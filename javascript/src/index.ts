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
  EndpointUpdate,
  EndpointSecretOut,
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
  EventTypeApi,
  ListResponseEventTypeOut,
  EventTypeIn,
  EventTypeOut,
  EventTypeUpdate,
  Middleware,
  RequestContext,
  ResponseContext,
} from "./openapi/index";
export * from "./openapi/models/all";
export * from "./openapi/apis/exception";
import * as utf8 from "@stablelib/utf8";
import * as base64 from "@stablelib/base64";
import * as sha256 from "fast-sha256";

const WEBHOOK_TOLERANCE_IN_SECONDS = 5 * 60; // 5 minutes
const VERSION = "0.30.0";

class UserAgentMiddleware implements Middleware {
  public pre(context: RequestContext): Promise<RequestContext> {
    context.setHeaderParam("User-Agent", `svix-libs/${VERSION}/javascript`);
    return Promise.resolve(context);
  }

  public post(context: ResponseContext): Promise<ResponseContext> {
    return Promise.resolve(context);
  }
}

export interface SvixOptions {
  debug?: boolean;
}

export class Svix {
  public readonly _configuration: Configuration;
  public readonly authentication: Authentication;
  public readonly application: Application;
  public readonly endpoint: Endpoint;
  public readonly eventType: EventType;
  public readonly message: Message;
  public readonly messageAttempt: MessageAttempt;

  public constructor(token: string, options: SvixOptions = {}) {
    const baseUrl: string = (options as any)._testUrl ?? "https://api.svix.com";

    const baseServer = new ServerConfiguration<any>(baseUrl, {});

    const bearerConfiguration: HttpBearerConfiguration = {
      tokenProvider: {
        getToken: () => token,
      },
    };
    const config = createConfiguration({
      baseServer,
      promiseMiddleware: [new UserAgentMiddleware()],
      authMethods: {
        HTTPBearer: bearerConfiguration,
      },
    });

    this._configuration = config;
    this.authentication = new Authentication(config);
    this.application = new Application(config);
    this.endpoint = new Endpoint(config);
    this.eventType = new EventType(config);
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

interface ListOptions {
  iterator?: string;
  limit?: number;
}

export type ApplicationListOptions = ListOptions;

export type EndpointListOptions = ListOptions;

export interface EventTypeListOptions extends ListOptions {
  withContent?: boolean;
}

export interface MessageListOptions extends ListOptions {
  eventTypes?: string[];
  before?: Date;
}

export interface MessageAttemptListOptions extends ListOptions {
  status?: MessageStatus;
  eventTypes?: string[];
  before?: Date;
}

class Application {
  private readonly api: ApplicationApi;

  public constructor(config: Configuration) {
    this.api = new ApplicationApi(config);
  }

  public list(options?: ApplicationListOptions): Promise<ListResponseApplicationOut> {
    return this.api.listApplicationsApiV1AppGet({ ...options });
  }

  public create(applicationIn: ApplicationIn): Promise<ApplicationOut> {
    return this.api.createApplicationApiV1AppPost({ applicationIn });
  }

  public get(appId: string): Promise<ApplicationOut> {
    return this.api.getApplicationApiV1AppAppIdGet({ appId });
  }

  public update(appId: string, applicationIn: ApplicationIn): Promise<ApplicationOut> {
    return this.api.updateApplicationApiV1AppAppIdPut({ appId, applicationIn });
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

  public list(
    appId: string,
    options?: EndpointListOptions
  ): Promise<ListResponseEndpointOut> {
    return this.api.listEndpointsApiV1AppAppIdEndpointGet({ appId, ...options });
  }

  public create(appId: string, endpointIn: EndpointIn): Promise<EndpointOut> {
    return this.api.createEndpointApiV1AppAppIdEndpointPost({ appId, endpointIn });
  }

  public get(appId: string, endpointId: string): Promise<EndpointOut> {
    return this.api.getEndpointApiV1AppAppIdEndpointEndpointIdGet({ endpointId, appId });
  }

  public update(
    appId: string,
    endpointId: string,
    endpointUpdate: EndpointUpdate
  ): Promise<EndpointOut> {
    return this.api.updateEndpointApiV1AppAppIdEndpointEndpointIdPut({
      appId,
      endpointId,
      endpointUpdate,
    });
  }

  public delete(appId: string, endpointId: string): Promise<void> {
    return this.api.deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete({
      endpointId,
      appId,
    });
  }

  public getSecret(appId: string, endpointId: string): Promise<EndpointSecretOut> {
    return this.api.getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet({
      endpointId,
      appId,
    });
  }
}

class EventType {
  private readonly api: EventTypeApi;

  public constructor(config: Configuration) {
    this.api = new EventTypeApi(config);
  }

  public list(options?: EventTypeListOptions): Promise<ListResponseEventTypeOut> {
    return this.api.listEventTypesApiV1EventTypeGet({ ...options });
  }

  public create(eventTypeIn: EventTypeIn): Promise<EventTypeOut> {
    return this.api.createEventTypeApiV1EventTypePost({ eventTypeIn });
  }

  public update(
    eventTypeName: string,
    eventTypeUpdate: EventTypeUpdate
  ): Promise<EventTypeOut> {
    return this.api.updateEventTypeApiV1EventTypeEventTypeNamePut({
      eventTypeName,
      eventTypeUpdate,
    });
  }

  public delete(eventTypeName: string): Promise<void> {
    return this.api.deleteEventTypeApiV1EventTypeEventTypeNameDelete({ eventTypeName });
  }
}

class Message {
  private readonly api: MessageApi;

  public constructor(config: Configuration) {
    this.api = new MessageApi(config);
  }

  public list(
    appId: string,
    options?: MessageListOptions
  ): Promise<ListResponseMessageOut> {
    return this.api.listMessagesApiV1AppAppIdMsgGet({ appId, ...options });
  }

  public create(appId: string, messageIn: MessageIn): Promise<MessageOut> {
    return this.api.createMessageApiV1AppAppIdMsgPost({ appId, messageIn });
  }

  public get(appId: string, msgId: string): Promise<MessageOut> {
    return this.api.getMessageApiV1AppAppIdMsgMsgIdGet({ msgId, appId });
  }
}

class MessageAttempt {
  private readonly api: MessageAttemptApi;

  public constructor(config: Configuration) {
    this.api = new MessageAttemptApi(config);
  }

  public list(
    appId: string,
    msgId: string,
    options?: MessageAttemptListOptions
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

  public resend(appId: string, msgId: string, endpointId: string): Promise<void> {
    return this.api.resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost({
      endpointId,
      msgId,
      appId,
    });
  }

  public listAttemptedMessages(
    appId: string,
    endpointId: string,
    options?: MessageAttemptListOptions
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
    options?: MessageAttemptListOptions
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
    options?: MessageAttemptListOptions
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
  "svix-id": string;
  "svix-timestamp": string;
  "svix-signature": string;
}

export interface WebhookUnbrandedRequiredHeaders {
  "webhook-id": string;
  "webhook-timestamp": string;
  "webhook-signature": string;
}

export class Webhook {
  private static prefix = "whsec_";
  private readonly key: Uint8Array;

  constructor(secret: string) {
    if (secret.startsWith(Webhook.prefix)) {
      secret = secret.substr(Webhook.prefix.length);
    }
    this.key = base64.decode(secret);
  }

  public verify(
    payload: string,
    headers_:
      | WebhookRequiredHeaders
      | WebhookUnbrandedRequiredHeaders
      | Record<string, string>
  ): unknown {
    const headers: Record<string, string> = {};
    for (const key of Object.keys(headers_)) {
      headers[key.toLowerCase()] = (headers_ as Record<string, string>)[key];
    }

    let msgId = headers["svix-id"];
    let msgSignature = headers["svix-signature"];
    let msgTimestamp = headers["svix-timestamp"];

    if (!msgSignature || !msgId || !msgTimestamp) {
      msgId = headers["webhook-id"];
      msgSignature = headers["webhook-signature"];
      msgTimestamp = headers["webhook-timestamp"];

      if (!msgSignature || !msgId || !msgTimestamp) {
        throw new WebhookVerificationError("Missing required headers");
      }
    }

    const timestamp = this.verifyTimestamp(msgTimestamp);

    const computedSignature = this.sign(msgId, timestamp, payload);
    const expectedSignature = computedSignature.split(",")[1];

    const passedSignatures = msgSignature.split(" ");
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

  public sign(msgId: string, timestamp: Date, payload: string): string {
    const toSign = utf8.encode(`${msgId}.${timestamp.getTime() / 1000}.${payload}`);
    const expectedSignature = base64.encode(sha256.hmac(this.key, toSign));
    return `v1,${expectedSignature}`;
  }

  private verifyTimestamp(timestampHeader: string): Date {
    const now = Math.floor(Date.now() / 1000);
    const timestamp = parseInt(timestampHeader, 10);
    if (isNaN(timestamp)) {
      throw new WebhookVerificationError("Invalid Signature Headers");
    }

    if (now - timestamp > WEBHOOK_TOLERANCE_IN_SECONDS) {
      throw new WebhookVerificationError("Message timestamp too old");
    }
    if (timestamp > now + WEBHOOK_TOLERANCE_IN_SECONDS) {
      throw new WebhookVerificationError("Message timestamp too new");
    }
    return new Date(timestamp * 1000);
  }
}
