import { Authentication } from "./api/authentication";
import { Application } from "./api/application";
import { BackgroundTask } from "./api/background_task";
import { Endpoint } from "./api/endpoint";
import { EventType } from "./api/event_type";
import { Integration } from "./api/integration";
import { Message } from "./api/message";
import { MessageAttempt } from "./api/message_attempt";
import { OperationalWebhookEndpoint } from "./api/operational_webhook_endpoint";
import { Statistics } from "./api/statistics";
import { LIB_VERSION, SvixRequestContext } from "./request";
import * as openapi from "./openapi";

export * from "./openapi/models/all";
export * from "./openapi/apis/exception";

export { PostOptions } from "./util";
export * from "./webhook";

export { ApplicationListOptions } from "./api/application";
export { BackgroundTaskListOptions } from "./api/background_task";
export { EndpointListOptions, EndpointGetStatsOptions } from "./api/endpoint";
export { EventTypeListOptions } from "./api/event_type";
export { IntegrationListOptions } from "./api/integration";
export { MessageListOptions, messageInRaw } from "./api/message";
export {
  MessageAttemptListByEndpointOptions,
  MessageAttemptListOptions,
} from "./api/message_attempt";
export { OperationalWebhookEndpointListOptions } from "./api/operational_webhook_endpoint";

export interface SvixOptions {
  debug?: boolean;
  serverUrl?: string;
  /** Time in milliseconds to wait for requests to get a response. */
  requestTimeout?: number;
}

const REGIONS = [
  { region: "us", url: "https://api.us.svix.com" },
  { region: "eu", url: "https://api.eu.svix.com" },
  { region: "in", url: "https://api.in.svix.com" },
];

class UserAgentMiddleware implements openapi.Middleware {
  public pre(context: openapi.RequestContext): Promise<openapi.RequestContext> {
    context.setHeaderParam("User-Agent", `svix-libs/${LIB_VERSION}/javascript`);
    return Promise.resolve(context);
  }

  public post(context: openapi.ResponseContext): Promise<openapi.ResponseContext> {
    return Promise.resolve(context);
  }
}

export class Svix {
  private readonly requestCtx: SvixRequestContext;
  public readonly _configuration: openapi.Configuration;

  public constructor(token: string, options: SvixOptions = {}) {
    const regionalUrl = REGIONS.find((x) => x.region === token.split(".")[1])?.url;
    const baseUrl: string = options.serverUrl ?? regionalUrl ?? "https://api.svix.com";

    this.requestCtx = { baseUrl, token, timeout: options.requestTimeout };

    this._configuration = openapi.createConfiguration({
      baseServer: new openapi.ServerConfiguration<any>(baseUrl, {}),
      promiseMiddleware: [new UserAgentMiddleware()],
      authMethods: {
        HTTPBearer: {
          tokenProvider: {
            getToken: () => token,
          },
        },
      },
    });
  }

  public get authentication() {
    return new Authentication(this.requestCtx);
  }

  public get application() {
    return new Application(this.requestCtx);
  }

  public get endpoint() {
    return new Endpoint(this.requestCtx);
  }

  public get eventType() {
    return new EventType(this.requestCtx);
  }

  public get integration() {
    return new Integration(this.requestCtx);
  }

  public get message() {
    return new Message(this.requestCtx);
  }

  public get messageAttempt() {
    return new MessageAttempt(this.requestCtx);
  }

  public get backgroundTask() {
    return new BackgroundTask(this.requestCtx);
  }

  public get statistics() {
    return new Statistics(this.requestCtx);
  }

  public get operationalWebhookEndpoint() {
    return new OperationalWebhookEndpoint(this.requestCtx);
  }
}
