// this file is @generated (with the exception of getOrCreate)
import {
  Configuration,
  ApplicationApi,
  ApplicationIn,
  ApplicationOut,
  ApplicationPatch,
  ListResponseApplicationOut,
  Ordering,
} from "../openapi";
import { PostOptions } from "../util";

export interface ApplicationListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export class Application {
  private readonly api: ApplicationApi;

  public constructor(config: Configuration) {
    this.api = new ApplicationApi(config);
  }

  /** List of all the organization's applications. */
  public list(options?: ApplicationListOptions): Promise<ListResponseApplicationOut> {
    return this.api.v1ApplicationList({
      ...options,
      iterator: options?.iterator ?? undefined,
    });
  }

  /** Create a new application. */
  public create(
    applicationIn: ApplicationIn,
    options?: PostOptions
  ): Promise<ApplicationOut> {
    return this.api.v1ApplicationCreate({
      applicationIn,
      ...options,
    });
  }

  /** Get the application with the UID from `applicationIn`, or create it if it doesn't exist yet. */
  public getOrCreate(
    applicationIn: ApplicationIn,
    options?: PostOptions
  ): Promise<ApplicationOut> {
    return this.api.v1ApplicationCreate({
      applicationIn,
      ...options,
      getIfExists: true,
    });
  }

  /** Get an application. */
  public get(appId: string): Promise<ApplicationOut> {
    return this.api.v1ApplicationGet({
      appId,
    });
  }

  /** Update an application. */
  public update(appId: string, applicationIn: ApplicationIn): Promise<ApplicationOut> {
    return this.api.v1ApplicationUpdate({
      appId,
      applicationIn,
    });
  }

  /** Delete an application. */
  public delete(appId: string): Promise<void> {
    return this.api.v1ApplicationDelete({
      appId,
    });
  }

  /** Partially update an application. */
  public patch(
    appId: string,
    applicationPatch: ApplicationPatch
  ): Promise<ApplicationOut> {
    return this.api.v1ApplicationPatch({
      appId,
      applicationPatch,
    });
  }
}
