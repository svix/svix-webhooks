import {
  Configuration,
  EventTypeApi,
  ListResponseEventTypeOut,
  EventTypeIn,
  EventTypeOut,
  EventTypeUpdate,
  EventTypePatch,
  EventTypeImportOpenApiIn,
  EventTypeImportOpenApiOut,
  Ordering,
} from "../openapi";
import { PostOptions } from "../util";

export interface EventTypeListOptions {
  /// Limit the number of returned items
  limit?: number;
  /// The iterator returned from a prior invocation
  iterator?: string | null;
  /// The sorting order of the returned items
  order?: Ordering;
  /// When `true` archived (deleted but not expunged) items are included in the response
  includeArchived?: boolean;
  /// When `true` the full item (including the schema) is included in the response
  withContent?: boolean;
}

export class EventType {
  private readonly api: EventTypeApi;

  public constructor(config: Configuration) {
    this.api = new EventTypeApi(config);
  }

  public list(options?: EventTypeListOptions): Promise<ListResponseEventTypeOut> {
    const iterator = options?.iterator ?? undefined;
    return this.api.v1EventTypeList({ ...options, iterator });
  }

  public get(eventTypeName: string): Promise<EventTypeOut> {
    return this.api.v1EventTypeGet({ eventTypeName });
  }

  public create(eventTypeIn: EventTypeIn, options?: PostOptions): Promise<EventTypeOut> {
    return this.api.v1EventTypeCreate({ eventTypeIn, ...options });
  }

  public update(
    eventTypeName: string,
    eventTypeUpdate: EventTypeUpdate
  ): Promise<EventTypeOut> {
    return this.api.v1EventTypeUpdate({
      eventTypeName,
      eventTypeUpdate,
    });
  }

  public patch(
    eventTypeName: string,
    eventTypePatch: EventTypePatch
  ): Promise<EventTypeOut> {
    return this.api.v1EventTypePatch({
      eventTypeName,
      eventTypePatch,
    });
  }

  public delete(eventTypeName: string): Promise<void> {
    return this.api.v1EventTypeDelete({ eventTypeName });
  }

  public importOpenApi(
    eventTypeImportOpenApiIn: EventTypeImportOpenApiIn,
    options?: PostOptions
  ): Promise<EventTypeImportOpenApiOut> {
    return this.api.v1EventTypeImportOpenapi({
      eventTypeImportOpenApiIn,
      ...options,
    });
  }
}
