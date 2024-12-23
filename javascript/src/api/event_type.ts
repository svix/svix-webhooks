// this file is @generated (with minor manual changes)
import {
  Configuration,
  EventTypeApi,
  EventTypeImportOpenApiIn,
  EventTypeImportOpenApiOut,
  EventTypeIn,
  EventTypeOut,
  EventTypePatch,
  EventTypeUpdate,
  ListResponseEventTypeOut,
  Ordering,
} from "../openapi";
import { PostOptions } from "../util";

export interface EventTypeListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
  /** When `true` archived (deleted but not expunged) items are included in the response. */
  includeArchived?: boolean;
  /** When `true` the full item (including the schema) is included in the response. */
  withContent?: boolean;
}

export class EventType {
  private readonly api: EventTypeApi;

  public constructor(config: Configuration) {
    this.api = new EventTypeApi(config);
  }

  /** Return the list of event types. */
  public list(options?: EventTypeListOptions): Promise<ListResponseEventTypeOut> {
    return this.api.v1EventTypeList({
      ...options,
      iterator: options?.iterator ?? undefined,
    });
  }

  /**
   * Create new or unarchive existing event type.
   *
   * Unarchiving an event type will allow endpoints to filter on it and messages to be sent with it.
   * Endpoints filtering on the event type before archival will continue to filter on it.
   * This operation does not preserve the description and schemas.
   */
  public create(eventTypeIn: EventTypeIn, options?: PostOptions): Promise<EventTypeOut> {
    return this.api.v1EventTypeCreate({
      eventTypeIn,
      ...options,
    });
  }

  /**
   * Given an OpenAPI spec, create new or update existing event types.
   * If an existing `archived` event type is updated, it will be unarchived.
   *
   * The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
   * top-level.
   */
  public importOpenapi(
    eventTypeImportOpenApiIn: EventTypeImportOpenApiIn,
    options?: PostOptions
  ): Promise<EventTypeImportOpenApiOut> {
    return this.api.v1EventTypeImportOpenapi({
      eventTypeImportOpenApiIn,
      ...options,
    });
  }

  /** Get an event type. */
  public get(eventTypeName: string): Promise<EventTypeOut> {
    return this.api.v1EventTypeGet({
      eventTypeName,
    });
  }

  /** Update an event type. */
  public update(
    eventTypeName: string,
    eventTypeUpdate: EventTypeUpdate
  ): Promise<EventTypeOut> {
    return this.api.v1EventTypeUpdate({
      eventTypeName,
      eventTypeUpdate,
    });
  }

  /**
   * Archive an event type.
   *
   * Endpoints already configured to filter on an event type will continue to do so after archival.
   * However, new messages can not be sent with it and endpoints can not filter on it.
   * An event type can be unarchived with the
   * [create operation](#operation/create_event_type_api_v1_event_type__post).
   */
  public delete(eventTypeName: string): Promise<void> {
    return this.api.v1EventTypeDelete({
      eventTypeName,
    });
  }

  /** Partially update an event type. */
  public patch(
    eventTypeName: string,
    eventTypePatch: EventTypePatch
  ): Promise<EventTypeOut> {
    return this.api.v1EventTypePatch({
      eventTypeName,
      eventTypePatch,
    });
  }
}
