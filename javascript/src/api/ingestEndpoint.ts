// this file is @generated
import {
  IngestEndpointHeadersIn,
  IngestEndpointHeadersInSerializer,
} from "../models/ingestEndpointHeadersIn";
import {
  IngestEndpointHeadersOut,
  IngestEndpointHeadersOutSerializer,
} from "../models/ingestEndpointHeadersOut";
import { IngestEndpointIn, IngestEndpointInSerializer } from "../models/ingestEndpointIn";
import {
  IngestEndpointOut,
  IngestEndpointOutSerializer,
} from "../models/ingestEndpointOut";
import {
  IngestEndpointSecretIn,
  IngestEndpointSecretInSerializer,
} from "../models/ingestEndpointSecretIn";
import {
  IngestEndpointSecretOut,
  IngestEndpointSecretOutSerializer,
} from "../models/ingestEndpointSecretOut";
import {
  IngestEndpointUpdate,
  IngestEndpointUpdateSerializer,
} from "../models/ingestEndpointUpdate";
import {
  ListResponseIngestEndpointOut,
  ListResponseIngestEndpointOutSerializer,
} from "../models/listResponseIngestEndpointOut";
import { Ordering } from "../models/ordering";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

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
    options?: IngestEndpointListOptions
  ): Promise<ListResponseIngestEndpointOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/ingest/api/v1/source/{source_id}/endpoint"
    );

    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("order", options?.order);

    return request.send(
      this.requestCtx,
      ListResponseIngestEndpointOutSerializer._fromJsonObject
    );
  }

  /** Create an ingest endpoint. */
  public create(
    ingestEndpointIn: IngestEndpointIn,
    options?: IngestEndpointCreateOptions
  ): Promise<IngestEndpointOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/ingest/api/v1/source/{source_id}/endpoint"
    );

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(IngestEndpointInSerializer._toJsonObject(ingestEndpointIn));

    return request.send(this.requestCtx, IngestEndpointOutSerializer._fromJsonObject);
  }

  /** Get an ingest endpoint. */
  public get(endpointId: string): Promise<IngestEndpointOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}"
    );

    request.setPathParam("endpoint_id", endpointId);

    return request.send(this.requestCtx, IngestEndpointOutSerializer._fromJsonObject);
  }

  /** Update an ingest endpoint. */
  public update(
    endpointId: string,
    ingestEndpointUpdate: IngestEndpointUpdate
  ): Promise<IngestEndpointOut> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}"
    );

    request.setPathParam("endpoint_id", endpointId);
    request.setBody(IngestEndpointUpdateSerializer._toJsonObject(ingestEndpointUpdate));

    return request.send(this.requestCtx, IngestEndpointOutSerializer._fromJsonObject);
  }

  /** Delete an ingest endpoint. */
  public delete(endpointId: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}"
    );

    request.setPathParam("endpoint_id", endpointId);

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Get the additional headers to be sent with the ingest. */
  public getHeaders(endpointId: string): Promise<IngestEndpointHeadersOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/headers"
    );

    request.setPathParam("endpoint_id", endpointId);

    return request.send(
      this.requestCtx,
      IngestEndpointHeadersOutSerializer._fromJsonObject
    );
  }

  /** Set the additional headers to be sent to the endpoint. */
  public updateHeaders(
    endpointId: string,
    ingestEndpointHeadersIn: IngestEndpointHeadersIn
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/headers"
    );

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
  public getSecret(endpointId: string): Promise<IngestEndpointSecretOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/secret"
    );

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
    endpointId: string,
    ingestEndpointSecretIn: IngestEndpointSecretIn,
    options?: IngestEndpointRotateSecretOptions
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/secret/rotate"
    );

    request.setPathParam("endpoint_id", endpointId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(
      IngestEndpointSecretInSerializer._toJsonObject(ingestEndpointSecretIn)
    );

    return request.sendNoResponseBody(this.requestCtx);
  }
}
