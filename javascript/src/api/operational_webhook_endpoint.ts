// this file is @generated
import {
  Configuration,
  WebhookEndpointApi,
  ListResponseOperationalWebhookEndpointOut,
  OperationalWebhookEndpointIn,
  OperationalWebhookEndpointOut,
  OperationalWebhookEndpointSecretIn,
  OperationalWebhookEndpointSecretOut,
  OperationalWebhookEndpointUpdate,
  Ordering,
} from "../openapi";
import { PostOptions } from "../util";

export interface OperationalWebhookEndpointListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export class OperationalWebhookEndpoint {
  private readonly api: WebhookEndpointApi;

  public constructor(config: Configuration) {
    this.api = new WebhookEndpointApi(config);
  }

  /** List operational webhook endpoints. */
  public list(
    options?: OperationalWebhookEndpointListOptions
  ): Promise<ListResponseOperationalWebhookEndpointOut> {
    return this.api.listOperationalWebhookEndpoints({
      ...options,
      iterator: options?.iterator ?? undefined,
    });
  }

  /** Create an operational webhook endpoint. */
  public create(
    operationalWebhookEndpointIn: OperationalWebhookEndpointIn,
    options?: PostOptions
  ): Promise<OperationalWebhookEndpointOut> {
    return this.api.createOperationalWebhookEndpoint({
      operationalWebhookEndpointIn,
      ...options,
    });
  }

  /** Get an operational webhook endpoint. */
  public get(endpointId: string): Promise<OperationalWebhookEndpointOut> {
    return this.api.getOperationalWebhookEndpoint({
      endpointId,
    });
  }

  /** Update an operational webhook endpoint. */
  public update(
    endpointId: string,
    operationalWebhookEndpointUpdate: OperationalWebhookEndpointUpdate
  ): Promise<OperationalWebhookEndpointOut> {
    return this.api.updateOperationalWebhookEndpoint({
      endpointId,
      operationalWebhookEndpointUpdate,
    });
  }

  /** Delete an operational webhook endpoint. */
  public delete(endpointId: string): Promise<void> {
    return this.api.deleteOperationalWebhookEndpoint({
      endpointId,
    });
  }

  /**
   * Get an operational webhook endpoint's signing secret.
   *
   * This is used to verify the authenticity of the webhook.
   * For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
   */
  public getSecret(endpointId: string): Promise<OperationalWebhookEndpointSecretOut> {
    return this.api.getOperationalWebhookEndpointSecret({
      endpointId,
    });
  }

  /**
   * Rotates an operational webhook endpoint's signing secret.
   *
   * The previous secret will remain valid for the next 24 hours.
   */
  public rotateSecret(
    endpointId: string,
    operationalWebhookEndpointSecretIn: OperationalWebhookEndpointSecretIn,
    options?: PostOptions
  ): Promise<void> {
    return this.api.rotateOperationalWebhookEndpointSecret({
      endpointId,
      operationalWebhookEndpointSecretIn,
      ...options,
    });
  }
}
