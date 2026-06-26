// this file is @generated

import { type IngestSourceIn, IngestSourceInSerializer } from "../models/ingestSourceIn";
import {
  type IngestSourceOut,
  IngestSourceOutSerializer,
} from "../models/ingestSourceOut";
import {
  type ListResponseIngestSourceOut,
  ListResponseIngestSourceOutSerializer,
} from "../models/listResponseIngestSourceOut";
import type { Ordering } from "../models/ordering";
import { type RotateTokenOut, RotateTokenOutSerializer } from "../models/rotateTokenOut";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export interface IngestSourceListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface IngestSourceCreateOptions {
  idempotencyKey?: string;
}

export interface IngestSourceRotateTokenOptions {
  idempotencyKey?: string;
}

export class IngestSource {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List of all the organization's Ingest Sources. */
  public async list(
    options?: IngestSourceListOptions
  ): Promise<ListResponseIngestSourceOut> {
    const request = new SvixRequest(HttpMethod.GET, "/ingest/api/v1/source");

    request.setQueryParams({
      limit: options?.limit,
      iterator: options?.iterator,
      order: options?.order,
    });

    return await request.send(
      this.requestCtx,
      ListResponseIngestSourceOutSerializer._fromJsonObject
    );
  }

  /** Create Ingest Source. */
  public async create(
    ingestSourceIn: IngestSourceIn,
    options?: IngestSourceCreateOptions
  ): Promise<IngestSourceOut> {
    const request = new SvixRequest(HttpMethod.POST, "/ingest/api/v1/source");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(IngestSourceInSerializer._toJsonObject(ingestSourceIn));

    return await request.send(this.requestCtx, IngestSourceOutSerializer._fromJsonObject);
  }

  /** Get an Ingest Source by id or uid. */
  public async get(sourceId: string): Promise<IngestSourceOut> {
    const request = new SvixRequest(HttpMethod.GET, "/ingest/api/v1/source/{source_id}");

    request.setPathParam("source_id", sourceId);

    return await request.send(this.requestCtx, IngestSourceOutSerializer._fromJsonObject);
  }

  /** Update an Ingest Source. */
  public async update(
    sourceId: string,
    ingestSourceIn: IngestSourceIn
  ): Promise<IngestSourceOut> {
    const request = new SvixRequest(HttpMethod.PUT, "/ingest/api/v1/source/{source_id}");

    request.setPathParam("source_id", sourceId);
    request.setBody(IngestSourceInSerializer._toJsonObject(ingestSourceIn));

    return await request.send(this.requestCtx, IngestSourceOutSerializer._fromJsonObject);
  }

  /** Delete an Ingest Source. */
  public async delete(sourceId: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/ingest/api/v1/source/{source_id}"
    );

    request.setPathParam("source_id", sourceId);

    return await request.sendNoResponseBody(this.requestCtx);
  }

  /**
   * Rotate the Ingest Source's Url Token.
   *
   * This will rotate the ingest source's token, which is used to
   * construct the unique `ingestUrl` for the source. Previous tokens
   * will remain valid for 48 hours after rotation. The token can be
   * rotated a maximum of three times within the 48-hour period.
   */
  public async rotateToken(
    sourceId: string,
    options?: IngestSourceRotateTokenOptions
  ): Promise<RotateTokenOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/ingest/api/v1/source/{source_id}/token/rotate"
    );

    request.setPathParam("source_id", sourceId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);

    return await request.send(this.requestCtx, RotateTokenOutSerializer._fromJsonObject);
  }
}
