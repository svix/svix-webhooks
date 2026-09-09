// this file is @generated

import { type DestinationIn, DestinationInSerializer } from "../models/destinationIn";
import { type DestinationOut, DestinationOutSerializer } from "../models/destinationOut";
import {
  type DestinationPatch,
  DestinationPatchSerializer,
} from "../models/destinationPatch";
import {
  type ListResponseDestinationOut,
  ListResponseDestinationOutSerializer,
} from "../models/listResponseDestinationOut";
import type { Ordering } from "../models/ordering";
import { DestinationTransformation } from "./destinationTransformation";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export interface DestinationListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface DestinationCreateOptions {
  idempotencyKey?: string;
}

export class Destination {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  public get transformation() {
    return new DestinationTransformation(this.requestCtx);
  }

  /** List of all the application's destinations. */
  public async list(
    appId: string,
    options?: DestinationListOptions
  ): Promise<ListResponseDestinationOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/app/{app_id}/destination");

    request.setPathParam("app_id", appId);
    request.setQueryParams({
      limit: options?.limit,
      iterator: options?.iterator,
      order: options?.order,
    });

    return await request.send(
      this.requestCtx,
      ListResponseDestinationOutSerializer._fromJsonObject
    );
  }

  /** Creates a new destination. */
  public async create(
    appId: string,
    destinationIn: DestinationIn,
    options?: DestinationCreateOptions
  ): Promise<DestinationOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/app/{app_id}/destination");

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(DestinationInSerializer._toJsonObject(destinationIn));

    return await request.send(this.requestCtx, DestinationOutSerializer._fromJsonObject);
  }

  /** Get a destination by id or uid. */
  public async get(appId: string, destinationId: string): Promise<DestinationOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/destination/{destination_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("destination_id", destinationId);

    return await request.send(this.requestCtx, DestinationOutSerializer._fromJsonObject);
  }

  /** Create or update a destination. */
  public async upsert(
    appId: string,
    destinationId: string,
    destinationIn: DestinationIn
  ): Promise<DestinationOut> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/app/{app_id}/destination/{destination_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("destination_id", destinationId);
    request.setBody(DestinationInSerializer._toJsonObject(destinationIn));

    return await request.send(this.requestCtx, DestinationOutSerializer._fromJsonObject);
  }

  /** Delete a destination. */
  public async delete(appId: string, destinationId: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/app/{app_id}/destination/{destination_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("destination_id", destinationId);

    return await request.sendNoResponseBody(this.requestCtx);
  }

  /** Partially update a destination. */
  public async patch(
    appId: string,
    destinationId: string,
    destinationPatch: DestinationPatch
  ): Promise<DestinationOut> {
    const request = new SvixRequest(
      HttpMethod.PATCH,
      "/api/v1/app/{app_id}/destination/{destination_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("destination_id", destinationId);
    request.setBody(DestinationPatchSerializer._toJsonObject(destinationPatch));

    return await request.send(this.requestCtx, DestinationOutSerializer._fromJsonObject);
  }
}
