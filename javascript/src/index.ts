import {
  ServerConfiguration,
  HttpBearerConfiguration,
  Configuration,
  createConfiguration,
  ApplicationApi,
  ApplicationOut,
  ListResponseApplicationOut,
  ApplicationIn,
  EndpointApi,
  ListResponseEndpointOut,
  EndpointOut,
  EndpointIn,
  EndpointSecret,
  MessageApi,
  MessageOut,
  MessageIn,
  ListResponseMessageOut,
  AuthenticationApi,
  DashboardAccessOut,
  MessageAttemptApi,
  ListResponseEndpointMessageOut,
  ListResponseMessageEndpointOut,
  ListResponseMessageAttemptEndpointOut,
  ListResponseMessageAttemptOut,
  MessageAttemptOut,
  MessageStatus,
} from "./openapi/index";
export * from "./openapi/models/all";
export * from "./openapi/apis/exception";
import { server1 } from "./openapi/servers";
import * as utf8 from "@stablelib/utf8";
import * as base64 from "@stablelib/base64";
import * as sha256 from "fast-sha256";

export interface DiahookOptions {
  debug?: boolean;
}

export class Diahook {
  public readonly _configuration: Configuration;
  public readonly authentication: Authentication;
  public readonly application: Application;
  public readonly endpoint: Endpoint;
  public readonly message: Message;
  public readonly messageAttempt: MessageAttempt;

  public constructor(token: string, options?: DiahookOptions) {
    const testUrl: string | undefined = (options as any)._testUrl;

    const baseServer = testUrl ? new ServerConfiguration<any>(testUrl, {}) : server1;

    const bearerConfiguration: HttpBearerConfiguration = {
      tokenProvider: {
        getToken: () => token,
      },
    };
    const config = createConfiguration({
      baseServer,
      authMethods: {
        HTTPBearer: bearerConfiguration,
      },
    });

    this._configuration = config;
    this.authentication = new Authentication(config);
    this.application = new Application(config);
    this.endpoint = new Endpoint(config);
    this.message = new Message(config);
    this.messageAttempt = new MessageAttempt(config);
  }
}

class Authentication {
  private readonly api: AuthenticationApi;

  public constructor(config: Configuration) {
    this.api = new AuthenticationApi(config);
  }

  public dashboardAccess(appId: string): Promise<DashboardAccessOut> {
    return this.api.getDashboardAccessApiV1AuthDashboardAccessAppIdPost({ appId });
  }

  public logout(): Promise<void> {
    return this.api.logoutApiV1AuthLogoutPost({});
  }
}

export interface FetchOptions {
  iterator?: string;
  limit?: number;
}

class Application {
  private readonly api: ApplicationApi;

  public constructor(config: Configuration) {
    this.api = new ApplicationApi(config);
  }

  public list(options?: FetchOptions): Promise<ListResponseApplicationOut> {
    return this.api.listApplicationsApiV1AppGet({ ...options });
  }

  public create(applicationIn: ApplicationIn): Promise<ApplicationOut> {
    return this.api.createApplicationApiV1AppPost({ applicationIn });
  }

  public get(appId: string): Promise<ApplicationOut> {
    return this.api.getApplicationApiV1AppAppIdGet({ appId });
  }

  public delete(appId: string): Promise<void> {
    return this.api.deleteApplicationApiV1AppAppIdDelete({ appId });
  }
}

class Endpoint {
  private readonly api: EndpointApi;

  public constructor(config: Configuration) {
    this.api = new EndpointApi(config);
  }

  public list(appId: string, options?: FetchOptions): Promise<ListResponseEndpointOut> {
    return this.api.listEndpointsApiV1AppAppIdEndpointGet({ appId, ...options });
  }

  public create(appId: string, endpointIn: EndpointIn): Promise<EndpointOut> {
    return this.api.createEndpointApiV1AppAppIdEndpointPost({ appId, endpointIn });
  }

  public get(appId: string, endpointId: string): Promise<EndpointOut> {
    return this.api.getEndpointApiV1AppAppIdEndpointEndpointIdGet({ endpointId, appId });
  }

  public delete(appId: string, endpointId: string): Promise<void> {
    return this.api.deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete({
      endpointId,
      appId,
    });
  }

  public getSecret(appId: string, endpointId: string): Promise<EndpointSecret> {
    return this.api.getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet({
      endpointId,
      appId,
    });
  }
}

class Message {
  private readonly api: MessageApi;

  public constructor(config: Configuration) {
    this.api = new MessageApi(config);
  }

  public list(appId: string, options?: FetchOptions): Promise<ListResponseMessageOut> {
    return this.api.listMessagesApiV1AppAppIdMsgGet({ appId, ...options });
  }

  public create(appId: string, messageIn: MessageIn): Promise<MessageOut> {
    return this.api.createMessageApiV1AppAppIdMsgPost({ appId, messageIn });
  }

  public get(appId: string, msgId: string): Promise<MessageOut> {
    return this.api.getMessageApiV1AppAppIdMsgMsgIdGet({ msgId, appId });
  }
}

export interface FetchOptionsMessageAttempt extends FetchOptions {
  status?: MessageStatus;
}

class MessageAttempt {
  private readonly api: MessageAttemptApi;

  public constructor(config: Configuration) {
    this.api = new MessageAttemptApi(config);
  }

  public list(
    appId: string,
    msgId: string,
    options?: FetchOptionsMessageAttempt
  ): Promise<ListResponseMessageAttemptOut> {
    return this.api.listAttemptsApiV1AppAppIdMsgMsgIdAttemptGet({
      appId,
      msgId,
      ...options,
    });
  }

  public get(
    appId: string,
    msgId: string,
    attemptId: string
  ): Promise<MessageAttemptOut> {
    return this.api.getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet({
      attemptId,
      msgId,
      appId,
    });
  }

  public resend(
    appId: string,
    msgId: string,
    endpointId: string
  ): Promise<MessageAttemptOut> {
    return this.api.resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost({
      endpointId,
      msgId,
      appId,
    });
  }

  public listAttemptedMessages(
    appId: string,
    endpointId: string,
    options?: FetchOptionsMessageAttempt
  ): Promise<ListResponseEndpointMessageOut> {
    return this.api.listAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet({
      appId,
      endpointId,
      ...options,
    });
  }

  public listAttemptedDestinations(
    appId: string,
    msgId: string,
    options?: FetchOptionsMessageAttempt
  ): Promise<ListResponseMessageEndpointOut> {
    return this.api.listAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet({
      appId,
      msgId,
      ...options,
    });
  }

  public listAttemptsForEndpoint(
    appId: string,
    msgId: string,
    endpointId: string,
    options?: FetchOptionsMessageAttempt
  ): Promise<ListResponseMessageAttemptEndpointOut> {
    return this.api.listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(
      { appId, msgId, endpointId, ...options }
    );
  }
}

class ExtendableError extends Error {
  constructor(message: any) {
    super(message);
    Object.setPrototypeOf(this, ExtendableError.prototype);
    this.name = "ExtendableError";
    this.stack = new Error(message).stack;
  }
}

export class WebhookVerificationError extends ExtendableError {
  constructor(message: string) {
    super(message);
    Object.setPrototypeOf(this, WebhookVerificationError.prototype);
    this.name = "WebhookVerificationError";
  }
}

export interface WebhookRequiredHeaders {
  "dh-id": string;
  "dh-timestamp": string;
  "dh-signature": string;
}

export class Webhook {
  private readonly key: Uint8Array;

  constructor(secret: string) {
    this.key = base64.decode(secret);
  }

  public verify(
    payload: string,
    headers: WebhookRequiredHeaders | Record<string, string>
  ): unknown {
    if (!headers["dh-signature"] || !headers["dh-id"] || !headers["dh-timestamp"]) {
      throw new WebhookVerificationError("Missing required headers");
    }

    const toSign = utf8.encode(
      `${headers["dh-id"]}.${headers["dh-timestamp"]}.${payload}`
    );
    const expectedSignature = base64.encode(sha256.hmac(this.key, toSign));
    const passedSignatures = headers["dh-signature"].split(" ");
    for (const versionedSignature of passedSignatures) {
      const [version, signature] = versionedSignature.split(",");
      if (version !== "v1") {
        continue;
      }

      if (signature === expectedSignature) {
        return JSON.parse(payload);
      }
    }
    throw new WebhookVerificationError("No matching signature found");
  }
}
