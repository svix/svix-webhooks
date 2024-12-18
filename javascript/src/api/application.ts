import {
  ApplicationApi,
  ApplicationIn,
  ApplicationOut,
  ApplicationPatch,
  Configuration,
  ListResponseApplicationOut,
  Ordering,
} from "../openapi";
import { PostOptions } from "../util";

export interface ApplicationListOptions {
  /// Limit the number of returned items
  limit?: number;
  /// The iterator returned from a prior invocation
  iterator?: string | null;
  /// The sorting order of the returned items
  order?: Ordering;
}

export class Application {
  private readonly api: ApplicationApi;

  public constructor(config: Configuration) {
    this.api = new ApplicationApi(config);
  }

  public list(options?: ApplicationListOptions): Promise<ListResponseApplicationOut> {
    const iterator = options?.iterator ?? undefined;
    return this.api.v1ApplicationList({ ...options, iterator });
  }

  public create(
    applicationIn: ApplicationIn,
    options?: PostOptions
  ): Promise<ApplicationOut> {
    return this.api.v1ApplicationCreate({ applicationIn, ...options });
  }

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

  public get(appId: string): Promise<ApplicationOut> {
    return this.api.v1ApplicationGet({ appId });
  }

  public update(appId: string, applicationIn: ApplicationIn): Promise<ApplicationOut> {
    return this.api.v1ApplicationUpdate({ appId, applicationIn });
  }

  public patch(
    appId: string,
    applicationPatch: ApplicationPatch
  ): Promise<ApplicationOut> {
    return this.api.v1ApplicationPatch({ appId, applicationPatch });
  }

  public delete(appId: string): Promise<void> {
    return this.api.v1ApplicationDelete({ appId });
  }
}
