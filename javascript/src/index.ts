import {
  ServerConfiguration,
  HttpBearerConfiguration,
  Configuration,
  createConfiguration,
  Middleware,
  RequestContext,
  ResponseContext,
} from "./openapi";
import { Authentication } from "./api/authentication";
import { Application } from "./api/application";
import { BackgroundTask } from "./api/background_task";
import { Endpoint } from "./api/endpoint";
import { EventType } from "./api/event_type";
import { Integration } from "./api/integration";
import { Message } from "./api/message";
import { MessageAttempt } from "./api/message_attempt";
import { OperationalWebhookEndpoint } from "./api/op_webhook_endpoint";
import { Statistics } from "./api/statistics";

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
export { OperationalWebhookEndpointListOptions } from "./api/op_webhook_endpoint";

const VERSION = "1.45.1";

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
  serverUrl?: string;
}

const REGIONS = [
  { region: "us", url: "https://api.us.svix.com" },
  { region: "eu", url: "https://api.eu.svix.com" },
  { region: "in", url: "https://api.in.svix.com" },
];

export class Svix {
  public readonly _configuration: Configuration;
  public readonly authentication: Authentication;
  public readonly application: Application;
  public readonly endpoint: Endpoint;
  public readonly eventType: EventType;
  public readonly integration: Integration;
  public readonly message: Message;
  public readonly messageAttempt: MessageAttempt;
  public readonly backgroundTask: BackgroundTask;
  public readonly statistics: Statistics;
  public readonly operationalWebhookEndpoint: OperationalWebhookEndpoint;

  public constructor(token: string, options: SvixOptions = {}) {
    const regionalUrl = REGIONS.find((x) => x.region === token.split(".")[1])?.url;
    const baseUrl: string = options.serverUrl ?? regionalUrl ?? "https://api.svix.com";

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
    this.integration = new Integration(config);
    this.message = new Message(config);
    this.messageAttempt = new MessageAttempt(config);
    this.backgroundTask = new BackgroundTask(config);
    this.statistics = new Statistics(config);
    this.operationalWebhookEndpoint = new OperationalWebhookEndpoint(config);
  }
}
