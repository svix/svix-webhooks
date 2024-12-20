// this file is @generated (with minor manual changes)
import {
  Configuration,
  IntegrationApi,
  IntegrationIn,
  IntegrationKeyOut,
  IntegrationOut,
  IntegrationUpdate,
  ListResponseIntegrationOut,
  Ordering,
} from "../openapi";
import { PostOptions } from "../util";

export interface IntegrationListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export class Integration {
  private readonly api: IntegrationApi;

  public constructor(config: Configuration) {
    this.api = new IntegrationApi(config);
  }

  /** List the application's integrations. */
  public list(
    appId: string,
    options?: IntegrationListOptions
  ): Promise<ListResponseIntegrationOut> {
    return this.api.v1IntegrationList({
      appId,
      ...options,
      iterator: options?.iterator ?? undefined,
    });
  }

  /** Create an integration. */
  public create(
    appId: string,
    integrationIn: IntegrationIn,
    options?: PostOptions
  ): Promise<IntegrationOut> {
    return this.api.v1IntegrationCreate({
      appId,
      integrationIn,
      ...options,
    });
  }

  /** Get an integration. */
  public get(appId: string, integId: string): Promise<IntegrationOut> {
    return this.api.v1IntegrationGet({
      appId,
      integId,
    });
  }

  /** Update an integration. */
  public update(
    appId: string,
    integId: string,
    integrationUpdate: IntegrationUpdate
  ): Promise<IntegrationOut> {
    return this.api.v1IntegrationUpdate({
      appId,
      integId,
      integrationUpdate,
    });
  }

  /** Delete an integration. */
  public delete(appId: string, integId: string): Promise<void> {
    return this.api.v1IntegrationDelete({
      appId,
      integId,
    });
  }

  public getKey(appId: string, integId: string): Promise<IntegrationKeyOut> {
    return this.api.v1IntegrationGetKey({
      integId,
      appId,
    });
  }

  /** Rotate the integration's key. The previous key will be immediately revoked. */
  public rotateKey(
    appId: string,
    integId: string,
    options?: PostOptions
  ): Promise<IntegrationKeyOut> {
    return this.api.v1IntegrationRotateKey({
      appId,
      integId,
      ...options,
    });
  }
}
