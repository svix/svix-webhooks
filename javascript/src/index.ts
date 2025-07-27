// this file is @generated
import { Application } from "./api/application";
import { Authentication } from "./api/authentication";
import { BackgroundTask } from "./api/backgroundTask";
import { Endpoint } from "./api/endpoint";
import { Environment } from "./api/environment";
import { EventType } from "./api/eventType";
import { Health } from "./api/health";
import { Ingest } from "./api/ingest";
import { Integration } from "./api/integration";
import { Message } from "./api/message";
import { MessageAttempt } from "./api/messageAttempt";
import { OperationalWebhook } from "./api/operationalWebhook";
import { OperationalWebhookEndpoint } from "./api/operationalWebhookEndpoint";
import { Statistics } from "./api/statistics";
import { SvixRequestContext } from "./request";

export { PostOptions, ApiException } from "./util";
export { HTTPValidationError, HttpErrorOut, ValidationError } from "./HttpErrors";
export * from "./webhook";
export * from "./models/index";

export { ApplicationListOptions } from "./api/application";
export { BackgroundTaskListOptions } from "./api/backgroundTask";
export { EndpointListOptions, EndpointGetStatsOptions } from "./api/endpoint";
export { EventTypeListOptions } from "./api/eventType";
export { IntegrationListOptions } from "./api/integration";
export { MessageListOptions, messageInRaw } from "./api/message";
export { MessageAttemptListByEndpointOptions } from "./api/messageAttempt";
export { OperationalWebhookEndpointListOptions } from "./api/operationalWebhookEndpoint";

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
  { region: "ca", url: "https://api.ca.svix.com" },
  { region: "au", url: "https://api.au.svix.com" },
];

export class Svix {
  private readonly requestCtx: SvixRequestContext;

  public constructor(token: string, options: SvixOptions = {}) {
    const regionalUrl = REGIONS.find((x) => x.region === token.split(".")[1])?.url;
    const baseUrl: string = (
      options.serverUrl ??
      regionalUrl ??
      "https://api.svix.com"
    ).replace(/\/+$/, "");

    this.requestCtx = { baseUrl, token, timeout: options.requestTimeout };
  }

  public get application() {
    return new Application(this.requestCtx);
  }

  public get authentication() {
    return new Authentication(this.requestCtx);
  }

  public get backgroundTask() {
    return new BackgroundTask(this.requestCtx);
  }

  public get endpoint() {
    return new Endpoint(this.requestCtx);
  }

  public get environment() {
    return new Environment(this.requestCtx);
  }

  public get eventType() {
    return new EventType(this.requestCtx);
  }

  public get health() {
    return new Health(this.requestCtx);
  }

  public get ingest() {
    return new Ingest(this.requestCtx);
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

  public get operationalWebhook() {
    return new OperationalWebhook(this.requestCtx);
  }

  public get statistics() {
    return new Statistics(this.requestCtx);
  }

  public get operationalWebhookEndpoint() {
    return new OperationalWebhookEndpoint(this.requestCtx);
  }
}
