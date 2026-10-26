import { ApiException, type XOR } from "./util";
import type { HttpErrorOut, HTTPValidationError } from "./HttpErrors";

export const LIB_VERSION = "2.9.1";

function getUserAgent() {
  var fields = [`svix-libs/${LIB_VERSION}/javascript`];
  // `typeof`, not a bare reference: outside their own runtime these are
  // undeclared, and reading an undeclared binding throws a ReferenceError.
  if (typeof process !== "undefined") {
    if (process.version !== undefined) {
      fields.push(`node/${process.version}`);
    }
    if (process.platform !== undefined && process.arch !== undefined) {
      fields.push(`${process.platform}/${process.arch}`);
    }
  } else if (typeof navigator !== "undefined" && navigator.userAgent !== undefined) {
    fields.push(navigator.userAgent);
  }
  return fields.join(" ");
}

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
  timeout: number;
  /**
   * Custom fetch implementation to use for HTTP requests.
   * Useful for testing, adding custom middleware, or running in non-standard environments.
   */
  fetch?: typeof fetch;
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

export type SvixRequestContextOptions = {
  serverUrl?: string;
  requestTimeout?: number;
  fetch?: typeof fetch;
} & XOR<
  {
    retryScheduleInMs?: number[];
  },
  {
    numRetries?: number;
  }
>;

const REGIONS = [
  { region: "us", url: "https://api.us.svix.com" },
  { region: "eu", url: "https://api.eu.svix.com" },
  { region: "in", url: "https://api.in.svix.com" },
  { region: "ca", url: "https://api.ca.svix.com" },
  { region: "au", url: "https://api.au.svix.com" },
];

const DEFAULT_REQUEST_TIMEOUT_MS = 15_000;

/** Shared by `Svix` and `api_internal` so the latter need not import the package barrel. */
export function createSvixRequestContext(
  token: string,
  options: SvixRequestContextOptions = {}
): SvixRequestContext {
  const timeout = options.requestTimeout ?? DEFAULT_REQUEST_TIMEOUT_MS;
  const regionalUrl = REGIONS.find((x) => x.region === token.split(".")[1])?.url;
  const baseUrl: string =
    options.serverUrl?.replace(/\/+$/, "") ?? regionalUrl ?? "https://api.svix.com";

  if (options.retryScheduleInMs != null) {
    return {
      baseUrl,
      token,
      timeout,
      retryScheduleInMs: options.retryScheduleInMs,
      fetch: options.fetch,
    };
  }
  if (options.numRetries != null) {
    if (Number.isNaN(options.numRetries)) {
      throw new TypeError("numRetries must not be NaN");
    }
    return {
      baseUrl,
      token,
      timeout,
      numRetries: options.numRetries,
      fetch: options.fetch,
    };
  }
  return {
    baseUrl,
    token,
    timeout,
    fetch: options.fetch,
  };
}

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

  public setQueryParams(params: { [name: string]: QueryParameter }) {
    for (const [name, value] of Object.entries(params)) {
      this.setQueryParam(name, value);
    }
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
    } else if (Array.isArray(value)) {
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
    if (response.status === 204) {
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
      this.headerParams["idempotency-key"] = `auto_${crypto.randomUUID()}`;
    }

    const randomId = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);

    if (this.body != null) {
      this.headerParams["content-type"] = "application/json";
    }
    // Cloudflare Workers fail if the credentials option is used in a fetch call.
    // This work around that. Source:
    // https://github.com/cloudflare/workers-sdk/issues/2514#issuecomment-21.90.0014
    const isCredentialsSupported = "credentials" in Request.prototype;

    const response = await sendWithRetry(
      url,
      {
        method: this.method.toString(),
        body: this.body,
        headers: {
          accept: "application/json, */*;q=0.8",
          authorization: `Bearer ${ctx.token}`,
          "user-agent": getUserAgent(),
          "svix-req-id": randomId.toString(),
          ...this.headerParams,
        },
        credentials: isCredentialsSupported ? "same-origin" : undefined,
      },
      ctx.timeout,
      ctx.retryScheduleInMs,
      ctx.retryScheduleInMs?.[0],
      ctx.retryScheduleInMs?.length ?? ctx.numRetries,
      ctx.fetch
    );
    return filterResponseForErrors(response);
  }
}

async function filterResponseForErrors(response: Response): Promise<Response> {
  if (response.status < 300) {
    return response;
  }

  const responseBody = await response.text();

  // A 4xx can come from a proxy or load balancer in front of the API (e.g. an
  // HTML 413 page) or have an empty body. Parsing that would throw a
  // SyntaxError and lose the status code, so fall back to the raw text, as
  // for 5xx responses below.
  let parsedBody: unknown;
  try {
    parsedBody = JSON.parse(responseBody);
  } catch {
    throw new ApiException(response.status, responseBody, response.headers);
  }

  if (response.status === 422) {
    throw new ApiException<HTTPValidationError>(
      response.status,
      parsedBody as HTTPValidationError,
      response.headers
    );
  }

  if (response.status >= 400 && response.status <= 499) {
    throw new ApiException<HttpErrorOut>(
      response.status,
      parsedBody as HttpErrorOut,
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
  timeout: number,
  retryScheduleInMs?: number[],
  nextInterval = 50,
  triesLeft = 2,
  fetchImpl: typeof fetch = fetch,
  retryCount = 1
): Promise<Response> {
  const sleep = (interval: number) =>
    new Promise((resolve) => setTimeout(resolve, interval));

  try {
    // A fresh signal per attempt, so the timeout applies to each attempt separately
    // rather than to all attempts combined.
    const response = await fetchImpl(url, {
      ...init,
      signal: AbortSignal.timeout(timeout),
    });
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
  nextInterval = retryScheduleInMs?.[retryCount] ?? nextInterval * 2;
  return await sendWithRetry(
    url,
    init,
    timeout,
    retryScheduleInMs,
    nextInterval,
    --triesLeft,
    fetchImpl,
    ++retryCount
  );
}
