import {
  Configuration,
  Ordering,
  WebhookEndpointApi,
  OperationalWebhookEndpointIn,
  OperationalWebhookEndpointOut,
  OperationalWebhookEndpointSecretIn,
  OperationalWebhookEndpointSecretOut,
  OperationalWebhookEndpointUpdate,
  ListResponseOperationalWebhookEndpointOut,
} from "../openapi";
import { PostOptions } from "../util";

export interface OperationalWebhookEndpointListOptions {
  /// Limit the number of returned items
  limit?: number;
  /// The iterator returned from a prior invocation
  iterator?: string | null;
  /// The sorting order of the returned items
  order?: Ordering;
}

export class OperationalWebhookEndpoint {
  private readonly api: WebhookEndpointApi;

  public constructor(config: Configuration) {
    this.api = new WebhookEndpointApi(config);
  }

  public list(
    options?: OperationalWebhookEndpointListOptions
  ): Promise<ListResponseOperationalWebhookEndpointOut> {
    const iterator = options?.iterator ?? undefined;
    return this.api.listOperationalWebhookEndpoints({ ...options, iterator });
  }

  public create(
    endpointIn: OperationalWebhookEndpointIn,
    options?: PostOptions
  ): Promise<OperationalWebhookEndpointOut> {
    return this.api.createOperationalWebhookEndpoint({
      operationalWebhookEndpointIn: endpointIn,
      ...options,
    });
  }

  public get(endpointId: string): Promise<OperationalWebhookEndpointOut> {
    return this.api.getOperationalWebhookEndpoint({ endpointId });
  }

  public update(
    endpointId: string,
    endpointUpdate: OperationalWebhookEndpointUpdate
  ): Promise<OperationalWebhookEndpointOut> {
    return this.api.updateOperationalWebhookEndpoint({
      endpointId,
      operationalWebhookEndpointUpdate: endpointUpdate,
    });
  }

  public delete(endpointId: string): Promise<void> {
    return this.api.deleteOperationalWebhookEndpoint({ endpointId });
  }

  public getSecret(endpointId: string): Promise<OperationalWebhookEndpointSecretOut> {
    return this.api.getOperationalWebhookEndpointSecret({ endpointId });
  }

  public rotateSecret(
    endpointId: string,
    endpointSecretIn: OperationalWebhookEndpointSecretIn,
    options?: PostOptions
  ): Promise<void> {
    return this.api.rotateOperationalWebhookEndpointSecret({
      endpointId,
      operationalWebhookEndpointSecretIn: endpointSecretIn,
      ...options,
    });
  }
}
