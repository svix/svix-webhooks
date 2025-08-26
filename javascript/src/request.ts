import { ApiException, XOR } from "./util";
import { HttpErrorOut, HTTPValidationError } from "./HttpErrors";
import { v4 as uuidv4 } from "uuid";

export const LIB_VERSION = "1.75.0";
const USER_AGENT = `svix-libs/${LIB_VERSION}/javascript`;

export enum HttpMethod {
  GET = "GET",
  HEAD = "HEAD",
  POST = "POST",
  PUT = "PUT",
  DELETE = "DELETE",
  CONNECT = "CONNECT",
  OPTIONS = "OPTIONS",
  TRACE = "TRACE",
  PATCH = "PATCH",
}

export type SvixRequestContext = {
  /** The API base URL, like "https://api.svix.com" */
  baseUrl: string;
  /** The 'bearer' scheme access token */
  token: string;
  /** Time in milliseconds to wait for requests to get a response. */
  timeout?: number;
} & XOR<
  {
    /** List of delays (in milliseconds) to wait before each retry attempt.*/
    retryScheduleInMs?: number[];
  },
  {
    /** The number of times the client will retry if a server-side error
     *  or timeout is received.
     *  Default: 2
     */
    numRetries?: number;
  }
>;

type QueryParameter = string | boolean | number | Date | string[] | null | undefined;

export class SvixRequest {
  constructor(
    private readonly method: HttpMethod,
    private path: string
  ) {}

  private body?: string;
  private queryParams: Record<string, string> = {};
  private headerParams: Record<string, string> = {};

  public setPathParam(name: string, value: string) {
    const newPath = this.path.replace(`{${name}}`, encodeURIComponent(value));
    if (this.path === newPath) {
      throw new Error(`path parameter ${name} not found`);
    }
    this.path = newPath;
  }

  public setQueryParam(name: string, value: QueryParameter) {
    if (value === undefined || value === null) {
      return;
    }

    if (typeof value === "string") {
      this.queryParams[name] = value;
    } else if (typeof value === "boolean" || typeof value === "number") {
      this.queryParams[name] = value.toString();
    } else if (value instanceof Date) {
      this.queryParams[name] = value.toISOString();
    } else if (value instanceof Array) {
      if (value.length > 0) {
        this.queryParams[name] = value.join(",");
      }
    } else {
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      const _assert_unreachable: never = value;
      throw new Error(`query parameter ${name} has unsupported type`);
    }
  }

  public setHeaderParam(name: string, value?: string) {
    if (value === undefined) {
      return;
    }

    this.headerParams[name] = value;
  }
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  public setBody(value: any) {
    this.body = JSON.stringify(value);
  }

  /**
   * Send this request, returning the request body as a caller-specified type.
   *
   * If the server returns a 422 error, an `ApiException<HTTPValidationError>` is thrown.
   * If the server returns another 4xx error, an `ApiException<HttpErrorOut>` is thrown.
   *
   * If the server returns a 5xx error, the request is retried up to two times with exponential backoff.
   * If retries are exhausted, an `ApiException<HttpErrorOut>` is thrown.
   */
  public async send<R>(
    ctx: SvixRequestContext,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    parseResponseBody: (jsonObject: any) => R
  ): Promise<R> {
    const response = await this.sendInner(ctx);
    if (response.status == 204) {
      return <R>null;
    }
    const responseBody = await response.text();
    return parseResponseBody(JSON.parse(responseBody));
  }

  /** Same as `send`, but the response body is discarded, not parsed. */
  public async sendNoResponseBody(ctx: SvixRequestContext): Promise<void> {
    await this.sendInner(ctx);
  }

  private async sendInner(ctx: SvixRequestContext): Promise<Response> {
    const url = new URL(ctx.baseUrl + this.path);
    for (const [name, value] of Object.entries(this.queryParams)) {
      url.searchParams.set(name, value);
    }

    if (
      this.headerParams["idempotency-key"] === undefined &&
      this.method.toUpperCase() === "POST"
    ) {
      this.headerParams["idempotency-key"] = "auto_" + uuidv4();
    }

    const randomId = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);

    if (this.body != null) {
      this.headerParams["content-type"] = "application/json";
    }
    // Cloudflare Workers fail if the credentials option is used in a fetch call.
    // This work around that. Source:
    // https://github.com/cloudflare/workers-sdk/issues/2514#issuecomment-2178070014
    const isCredentialsSupported = "credentials" in Request.prototype;

    const response = await sendWithRetry(
      url,
      {
        method: this.method.toString(),
        body: this.body,
        headers: {
          accept: "application/json, */*;q=0.8",
          authorization: `Bearer ${ctx.token}`,
          "user-agent": USER_AGENT,
          "svix-req-id": randomId.toString(),
          ...this.headerParams,
        },
        credentials: isCredentialsSupported ? "same-origin" : undefined,
        signal: ctx.timeout !== undefined ? AbortSignal.timeout(ctx.timeout) : undefined,
      },
      ctx.retryScheduleInMs,
      ctx.retryScheduleInMs?.[0],
      ctx.retryScheduleInMs?.length || ctx.numRetries
    );
    return filterResponseForErrors(response);
  }
}

async function filterResponseForErrors(response: Response): Promise<Response> {
  if (response.status < 300) {
    return response;
  }

  const responseBody = await response.text();

  if (response.status === 422) {
    throw new ApiException<HTTPValidationError>(
      response.status,
      JSON.parse(responseBody) as HTTPValidationError,
      response.headers
    );
  }

  if (response.status >= 400 && response.status <= 499) {
    throw new ApiException<HttpErrorOut>(
      response.status,
      JSON.parse(responseBody) as HttpErrorOut,
      response.headers
    );
  }
  throw new ApiException(response.status, responseBody, response.headers);
}

type SvixRequestInit = RequestInit & {
  headers: Record<string, string>;
};

async function sendWithRetry(
  url: URL,
  init: SvixRequestInit,
  retryScheduleInMs?: number[],
  nextInterval = 50,
  triesLeft = 2,
  retryCount = 1
): Promise<Response> {
  const sleep = (interval: number) =>
    new Promise((resolve) => setTimeout(resolve, interval));

  try {
    const response = await fetch(url, init);
    if (triesLeft <= 0 || response.status < 500) {
      return response;
    }
  } catch (e) {
    if (triesLeft <= 0) {
      throw e;
    }
  }

  await sleep(nextInterval);
  init.headers["svix-retry-count"] = retryCount.toString();
  nextInterval = retryScheduleInMs?.[retryCount] || nextInterval * 2;
  return await sendWithRetry(
    url,
    init,
    retryScheduleInMs,
    nextInterval,
    --triesLeft,
    ++retryCount
  );
}
