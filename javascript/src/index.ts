// this file is @generated
import { Application } from "./api/application";
import { Authentication } from "./api/authentication";
import { BackgroundTask } from "./api/backgroundTask";
import { Connector } from "./api/connector";
import { Endpoint } from "./api/endpoint";
import { Environment } from "./api/environment";
import { EventType } from "./api/eventType";
import { Health } from "./api/health";
import { Ingest } from "./api/ingest";
import { Integration } from "./api/integration";
import { Message } from "./api/message";
import { MessageAttempt } from "./api/messageAttempt";
import { OperationalWebhook } from "./api/operationalWebhook";
import { Statistics } from "./api/statistics";
import { Streaming } from "./api/streaming";
import { createSvixRequestContext, type SvixRequestContext } from "./request";

export { type PostOptions, ApiException } from "./util";
export { HTTPValidationError, HttpErrorOut, ValidationError } from "./HttpErrors";
export * from "./webhook";
export * from "./models/index";
import type { XOR } from "./util";

export type { ApplicationListOptions } from "./api/application";
export type { BackgroundTaskListOptions } from "./api/backgroundTask";
export type { EndpointListOptions, EndpointGetStatsOptions } from "./api/endpoint";
export type { EventTypeListOptions } from "./api/eventType";
export type { IntegrationListOptions } from "./api/integration";
export { type MessageListOptions, messageInRaw } from "./api/message";
export type { MessageAttemptListByEndpointOptions } from "./api/messageAttempt";

export type SvixOptions = {
  debug?: boolean;
  serverUrl?: string;
  /** Time in milliseconds to wait for requests to get a response. */
  requestTimeout?: number;
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

export class Svix {
  protected readonly requestCtx: SvixRequestContext;

  public constructor(token: string, options: SvixOptions = {}) {
    this.requestCtx = createSvixRequestContext(token, options);
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

  public get connector() {
    return new Connector(this.requestCtx);
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

  public get streaming() {
    return new Streaming(this.requestCtx);
  }
}

// Last to avoid circular dependency
export { AutoConfig, AutoConfigError } from "./autoconfig";
export { AutoConfigConsumer } from "./autoconfigConsumer";
