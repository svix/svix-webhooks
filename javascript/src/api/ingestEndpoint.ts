// this file is @generated

import {
  type IngestEndpointHeadersIn,
  IngestEndpointHeadersInSerializer,
} from "../models/ingestEndpointHeadersIn";
import {
  type IngestEndpointHeadersOut,
  IngestEndpointHeadersOutSerializer,
} from "../models/ingestEndpointHeadersOut";
import {
  type IngestEndpointIn,
  IngestEndpointInSerializer,
} from "../models/ingestEndpointIn";
import {
  type IngestEndpointOut,
  IngestEndpointOutSerializer,
} from "../models/ingestEndpointOut";
import {
  type IngestEndpointSecretIn,
  IngestEndpointSecretInSerializer,
} from "../models/ingestEndpointSecretIn";
import {
  type IngestEndpointSecretOut,
  IngestEndpointSecretOutSerializer,
} from "../models/ingestEndpointSecretOut";
import {
  type IngestEndpointTransformationOut,
  IngestEndpointTransformationOutSerializer,
} from "../models/ingestEndpointTransformationOut";
import {
  type IngestEndpointTransformationPatch,
  IngestEndpointTransformationPatchSerializer,
} from "../models/ingestEndpointTransformationPatch";
import {
  type IngestEndpointUpdate,
  IngestEndpointUpdateSerializer,
} from "../models/ingestEndpointUpdate";
import {
  type ListResponseIngestEndpointOut,
  ListResponseIngestEndpointOutSerializer,
} from "../models/listResponseIngestEndpointOut";
import type { Ordering } from "../models/ordering";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export interface IngestEndpointListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface IngestEndpointCreateOptions {
  idempotencyKey?: string;
}

export interface IngestEndpointRotateSecretOptions {
  idempotencyKey?: string;
}

export class IngestEndpoint {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List ingest endpoints. */
  public list(
    sourceId: string,
    options?: IngestEndpointListOptions
  ): Promise<ListResponseIngestEndpointOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/ingest/api/v1/source/{source_id}/endpoint"
    );

    request.setPathParam("source_id", sourceId);
    request.setQueryParams({
      limit: options?.limit,
      iterator: options?.iterator,
      order: options?.order,
    });

    return request.send(
      this.requestCtx,
      ListResponseIngestEndpointOutSerializer._fromJsonObject
    );
  }

  /** Create an ingest endpoint. */
  public create(
    sourceId: string,
    ingestEndpointIn: IngestEndpointIn,
    options?: IngestEndpointCreateOptions
  ): Promise<IngestEndpointOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/ingest/api/v1/source/{source_id}/endpoint"
    );

    request.setPathParam("source_id", sourceId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(IngestEndpointInSerializer._toJsonObject(ingestEndpointIn));

    return request.send(this.requestCtx, IngestEndpointOutSerializer._fromJsonObject);
  }

  /** Get an ingest endpoint. */
  public get(sourceId: string, endpointId: string): Promise<IngestEndpointOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}"
    );

    request.setPathParam("source_id", sourceId);
    request.setPathParam("endpoint_id", endpointId);

    return request.send(this.requestCtx, IngestEndpointOutSerializer._fromJsonObject);
  }

  /** Update an ingest endpoint. */
  public update(
    sourceId: string,
    endpointId: string,
    ingestEndpointUpdate: IngestEndpointUpdate
  ): Promise<IngestEndpointOut> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}"
    );

    request.setPathParam("source_id", sourceId);
    request.setPathParam("endpoint_id", endpointId);
    request.setBody(IngestEndpointUpdateSerializer._toJsonObject(ingestEndpointUpdate));

    return request.send(this.requestCtx, IngestEndpointOutSerializer._fromJsonObject);
  }

  /** Delete an ingest endpoint. */
  public delete(sourceId: string, endpointId: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}"
    );

    request.setPathParam("source_id", sourceId);
    request.setPathParam("endpoint_id", endpointId);

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Get the additional headers to be sent with the ingest. */
  public getHeaders(
    sourceId: string,
    endpointId: string
  ): Promise<IngestEndpointHeadersOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/headers"
    );

    request.setPathParam("source_id", sourceId);
    request.setPathParam("endpoint_id", endpointId);

    return request.send(
      this.requestCtx,
      IngestEndpointHeadersOutSerializer._fromJsonObject
    );
  }

  /** Set the additional headers to be sent to the endpoint. */
  public updateHeaders(
    sourceId: string,
    endpointId: string,
    ingestEndpointHeadersIn: IngestEndpointHeadersIn
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/headers"
    );

    request.setPathParam("source_id", sourceId);
    request.setPathParam("endpoint_id", endpointId);
    request.setBody(
      IngestEndpointHeadersInSerializer._toJsonObject(ingestEndpointHeadersIn)
    );

    return request.sendNoResponseBody(this.requestCtx);
  }

  /**
   * Get an ingest endpoint's signing secret.
   *
   * This is used to verify the authenticity of the webhook.
   * For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
   */
  public getSecret(
    sourceId: string,
    endpointId: string
  ): Promise<IngestEndpointSecretOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/secret"
    );

    request.setPathParam("source_id", sourceId);
    request.setPathParam("endpoint_id", endpointId);

    return request.send(
      this.requestCtx,
      IngestEndpointSecretOutSerializer._fromJsonObject
    );
  }

  /**
   * Rotates an ingest endpoint's signing secret.
   *
   * The previous secret will remain valid for the next 24 hours.
   */
  public rotateSecret(
    sourceId: string,
    endpointId: string,
    ingestEndpointSecretIn: IngestEndpointSecretIn,
    options?: IngestEndpointRotateSecretOptions
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/secret/rotate"
    );

    request.setPathParam("source_id", sourceId);
    request.setPathParam("endpoint_id", endpointId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(
      IngestEndpointSecretInSerializer._toJsonObject(ingestEndpointSecretIn)
    );

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Get the transformation code associated with this ingest endpoint. */
  public getTransformation(
    sourceId: string,
    endpointId: string
  ): Promise<IngestEndpointTransformationOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation"
    );

    request.setPathParam("source_id", sourceId);
    request.setPathParam("endpoint_id", endpointId);

    return request.send(
      this.requestCtx,
      IngestEndpointTransformationOutSerializer._fromJsonObject
    );
  }

  /** Set or unset the transformation code associated with this ingest endpoint. */
  public setTransformation(
    sourceId: string,
    endpointId: string,
    ingestEndpointTransformationPatch: IngestEndpointTransformationPatch
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.PATCH,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation"
    );

    request.setPathParam("source_id", sourceId);
    request.setPathParam("endpoint_id", endpointId);
    request.setBody(
      IngestEndpointTransformationPatchSerializer._toJsonObject(
        ingestEndpointTransformationPatch
      )
    );

    return request.sendNoResponseBody(this.requestCtx);
  }
}
