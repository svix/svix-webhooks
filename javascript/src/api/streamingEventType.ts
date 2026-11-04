// this file is @generated

import type { ArchiveStatus } from "../models/archiveStatus";
import {
  type ListResponseStreamEventTypeOut,
  ListResponseStreamEventTypeOutSerializer,
} from "../models/listResponseStreamEventTypeOut";
import type { Ordering } from "../models/ordering";
import {
  type StreamEventTypeIn,
  StreamEventTypeInSerializer,
} from "../models/streamEventTypeIn";
import {
  type StreamEventTypeOut,
  StreamEventTypeOutSerializer,
} from "../models/streamEventTypeOut";
import {
  type StreamEventTypePatch,
  StreamEventTypePatchSerializer,
} from "../models/streamEventTypePatch";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export interface StreamingEventTypeListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
  /**
   * Include archived items in the response.
   *
   * Deprecated, use `archive_status` instead. Ignored when `archive_status` is set.
   */
  includeArchived?: boolean;
  /**
   * Which items to include based on whether they are archived.
   *
   * Defaults to `active`.
   */
  archiveStatus?: ArchiveStatus;
}

export interface StreamingEventTypeCreateOptions {
  idempotencyKey?: string;
}

export interface StreamingEventTypeDeleteOptions {
  /** Entirely delete the event type, rather than archiving it. */
  expunge?: boolean;
}

export class StreamingEventType {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List of all the organization's event types for streaming. */
  public async list(
    options?: StreamingEventTypeListOptions
  ): Promise<ListResponseStreamEventTypeOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/stream/event-type");

    request.setQueryParams({
      limit: options?.limit,
      iterator: options?.iterator,
      order: options?.order,
      include_archived: options?.includeArchived,
      archive_status: options?.archiveStatus,
    });

    return await request.send(
      this.requestCtx,
      ListResponseStreamEventTypeOutSerializer._fromJsonObject
    );
  }

  /** Create an event type for Streams. */
  public async create(
    streamEventTypeIn: StreamEventTypeIn,
    options?: StreamingEventTypeCreateOptions
  ): Promise<StreamEventTypeOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/stream/event-type");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(StreamEventTypeInSerializer._toJsonObject(streamEventTypeIn));

    return await request.send(
      this.requestCtx,
      StreamEventTypeOutSerializer._fromJsonObject
    );
  }

  /** Get an event type. */
  public async get(name: string): Promise<StreamEventTypeOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/stream/event-type/{name}");

    request.setPathParam("name", name);

    return await request.send(
      this.requestCtx,
      StreamEventTypeOutSerializer._fromJsonObject
    );
  }

  /** Create or update or create a event type for Streams. */
  public async upsert(
    name: string,
    streamEventTypeIn: StreamEventTypeIn
  ): Promise<StreamEventTypeOut> {
    const request = new SvixRequest(HttpMethod.PUT, "/api/v1/stream/event-type/{name}");

    request.setPathParam("name", name);
    request.setBody(StreamEventTypeInSerializer._toJsonObject(streamEventTypeIn));

    return await request.send(
      this.requestCtx,
      StreamEventTypeOutSerializer._fromJsonObject
    );
  }

  /**
   * Archive an event type.
   *
   * An event type can be unarchived with the
   * [patch operation](#operation/v1.streaming.event-type.patch) by setting `archived` to `false`.
   */
  public async delete(
    name: string,
    options?: StreamingEventTypeDeleteOptions
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/stream/event-type/{name}"
    );

    request.setPathParam("name", name);
    request.setQueryParams({
      expunge: options?.expunge,
    });

    return await request.sendNoResponseBody(this.requestCtx);
  }

  /** Patch an event type for Streams. */
  public async patch(
    name: string,
    streamEventTypePatch: StreamEventTypePatch
  ): Promise<StreamEventTypeOut> {
    const request = new SvixRequest(HttpMethod.PATCH, "/api/v1/stream/event-type/{name}");

    request.setPathParam("name", name);
    request.setBody(StreamEventTypePatchSerializer._toJsonObject(streamEventTypePatch));

    return await request.send(
      this.requestCtx,
      StreamEventTypeOutSerializer._fromJsonObject
    );
  }
}
