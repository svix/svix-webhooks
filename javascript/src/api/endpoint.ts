// this file is @generated (with manual changes)
import {
  Configuration,
  EndpointApi,
  EndpointHeadersIn,
  EndpointHeadersOut,
  EndpointHeadersPatchIn,
  EndpointIn,
  EndpointOauthConfigIn,
  EndpointOut,
  EndpointPatch,
  EndpointSecretOut,
  EndpointSecretRotateIn,
  EndpointStats,
  EndpointTransformationIn,
  EndpointTransformationOut,
  EndpointUpdate,
  EventExampleIn,
  ListResponseEndpointOut,
  MessageOut,
  Ordering,
  RecoverIn,
  RecoverOut,
  ReplayIn,
  ReplayOut,
} from "../openapi";
import { PostOptions } from "../util";

export interface EndpointListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface EndpointGetStatsOptions {
  /** Filter the range to data starting from this date. */
  since?: Date | null;
  /** Filter the range to data ending by this date. */
  until?: Date | null;
}

export class Endpoint {
  private readonly api: EndpointApi;

  public constructor(config: Configuration) {
    this.api = new EndpointApi(config);
  }

  /** List the application's endpoints. */
  public list(
    appId: string,
    options?: EndpointListOptions
  ): Promise<ListResponseEndpointOut> {
    return this.api.v1EndpointList({
      appId,
      ...options,
      iterator: options?.iterator ?? undefined,
    });
  }

  /**
   * Create a new endpoint for the application.
   *
   * When `secret` is `null` the secret is automatically generated (recommended).
   */
  public create(
    appId: string,
    endpointIn: EndpointIn,
    options?: PostOptions
  ): Promise<EndpointOut> {
    return this.api.v1EndpointCreate({
      appId,
      endpointIn,
      ...options,
    });
  }

  /** Get an endpoint. */
  public get(appId: string, endpointId: string): Promise<EndpointOut> {
    return this.api.v1EndpointGet({
      appId,
      endpointId,
    });
  }

  /** Update an endpoint. */
  public update(
    appId: string,
    endpointId: string,
    endpointUpdate: EndpointUpdate
  ): Promise<EndpointOut> {
    return this.api.v1EndpointUpdate({
      appId,
      endpointId,
      endpointUpdate,
    });
  }

  /** Delete an endpoint. */
  public delete(appId: string, endpointId: string): Promise<void> {
    return this.api.v1EndpointDelete({
      appId,
      endpointId,
    });
  }

  /** Partially update an endpoint. */
  public patch(
    appId: string,
    endpointId: string,
    endpointPatch: EndpointPatch
  ): Promise<EndpointOut> {
    return this.api.v1EndpointPatch({
      appId,
      endpointId,
      endpointPatch,
    });
  }

  /** Get the additional headers to be sent with the webhook. */
  public getHeaders(appId: string, endpointId: string): Promise<EndpointHeadersOut> {
    return this.api.v1EndpointGetHeaders({
      appId,
      endpointId,
    });
  }

  /** Set the additional headers to be sent with the webhook. */
  public updateHeaders(
    appId: string,
    endpointId: string,
    endpointHeadersIn: EndpointHeadersIn
  ): Promise<void> {
    return this.api.v1EndpointUpdateHeaders({
      appId,
      endpointId,
      endpointHeadersIn,
    });
  }

  public headersUpdate(
    appId: string,
    endpointId: string,
    endpointHeadersIn: EndpointHeadersIn
  ): Promise<void> {
    return this.api.v1EndpointUpdateHeaders({
      appId,
      endpointId,
      endpointHeadersIn,
    });
  }

  /**
   * @deprecated Since version 1.30.0. Use `headersPatch` instead.
   */
  public patchHeaders(
    appId: string,
    endpointId: string,
    endpointHeadersPatchIn: EndpointHeadersPatchIn
  ): Promise<void> {
    return this.api.v1EndpointPatchHeaders({
      appId,
      endpointId,
      endpointHeadersPatchIn,
    });
  }

  public headersPatch(
    appId: string,
    endpointId: string,
    endpointHeadersPatchIn: EndpointHeadersPatchIn
  ): Promise<void> {
    return this.api.v1EndpointPatchHeaders({
      appId,
      endpointId,
      endpointHeadersPatchIn,
    });
  }

  /**
   * Resend all failed messages since a given time.
   *
   * Messages that were sent successfully, even if failed initially, are not resent.
   */
  public recover(
    appId: string,
    endpointId: string,
    recoverIn: RecoverIn,
    options?: PostOptions
  ): Promise<RecoverOut> {
    return this.api.v1EndpointRecover({
      appId,
      endpointId,
      recoverIn,
      ...options,
    });
  }

  /**
   * Replays messages to the endpoint.
   *
   * Only messages that were created after `since` will be sent.
   * Messages that were previously sent to the endpoint are not resent.
   */
  public replayMissing(
    appId: string,
    endpointId: string,
    replayIn: ReplayIn,
    options?: PostOptions
  ): Promise<ReplayOut> {
    return this.api.v1EndpointReplayMissing({
      appId,
      endpointId,
      replayIn,
      ...options,
    });
  }

  /**
   * Get the endpoint's signing secret.
   *
   * This is used to verify the authenticity of the webhook.
   * For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
   */
  public getSecret(appId: string, endpointId: string): Promise<EndpointSecretOut> {
    return this.api.v1EndpointGetSecret({
      appId,
      endpointId,
    });
  }

  /**
   * Rotates the endpoint's signing secret.
   *
   * The previous secret will remain valid for the next 24 hours.
   */
  public rotateSecret(
    appId: string,
    endpointId: string,
    endpointSecretRotateIn: EndpointSecretRotateIn,
    options?: PostOptions
  ): Promise<void> {
    return this.api.v1EndpointRotateSecret({
      appId,
      endpointId,
      endpointSecretRotateIn,
      ...options,
    });
  }

  /** Send an example message for an event. */
  public sendExample(
    appId: string,
    endpointId: string,
    eventExampleIn: EventExampleIn,
    options?: PostOptions
  ): Promise<MessageOut> {
    return this.api.v1EndpointSendExample({
      appId,
      endpointId,
      eventExampleIn,
      ...options,
    });
  }

  /** Get basic statistics for the endpoint. */
  public getStats(
    appId: string,
    endpointId: string,
    options?: EndpointGetStatsOptions
  ): Promise<EndpointStats> {
    return this.api.v1EndpointGetStats({
      appId,
      endpointId,
      ...options,
      since: options?.since ?? undefined,
      until: options?.until ?? undefined,
    });
  }

  /** Get the transformation code associated with this endpoint. */
  public transformationGet(
    appId: string,
    endpointId: string
  ): Promise<EndpointTransformationOut> {
    return this.api.v1EndpointTransformationGet({
      appId,
      endpointId,
    });
  }

  /** Set or unset the transformation code associated with this endpoint. */
  public transformationPartialUpdate(
    appId: string,
    endpointId: string,
    endpointTransformationIn: EndpointTransformationIn
  ): Promise<void> {
    return this.api.v1EndpointTransformationPartialUpdate({
      appId,
      endpointId,
      endpointTransformationIn,
    });
  }

  public oauthUpdate(
    appId: string,
    endpointId: string,
    endpointOauthConfigIn: EndpointOauthConfigIn
  ): Promise<void> {
    return this.api.v1EndpointUpdateOauthConfig({
      appId,
      endpointId,
      endpointOauthConfigIn,
    });
  }

  public oauthDelete(appId: string, endpointId: string): Promise<void> {
    return this.api.v1EndpointDeleteOauthConfig({
      appId,
      endpointId,
    });
  }
}
