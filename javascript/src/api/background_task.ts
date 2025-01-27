// this file is @generated
import {
  Configuration,
  BackgroundTasksApi,
  BackgroundTaskOut,
  BackgroundTaskStatus,
  BackgroundTaskType,
  ListResponseBackgroundTaskOut,
  Ordering,
} from "../openapi";

export interface BackgroundTaskListOptions {
  /** Filter the response based on the status. */
  status?: BackgroundTaskStatus;
  /** Filter the response based on the type. */
  task?: BackgroundTaskType;
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export class BackgroundTask {
  private readonly api: BackgroundTasksApi;

  public constructor(config: Configuration) {
    this.api = new BackgroundTasksApi(config);
  }

  /** List background tasks executed in the past 90 days. */
  public list(
    options?: BackgroundTaskListOptions
  ): Promise<ListResponseBackgroundTaskOut> {
    return this.api.v1BackgroundTaskList({
      ...options,
      iterator: options?.iterator ?? undefined,
    });
  }

  /**
   * List background tasks executed in the past 90 days.
   *
   * @deprecated Use list instead.
   * */
  public listByEndpoint(
    options?: BackgroundTaskListOptions
  ): Promise<ListResponseBackgroundTaskOut> {
    return this.list(options);
  }

  /** Get a background task by ID. */
  public get(taskId: string): Promise<BackgroundTaskOut> {
    return this.api.v1BackgroundTaskGet({
      taskId,
    });
  }
}
