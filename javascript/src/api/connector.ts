// this file is @generated

import { type ConnectorIn, ConnectorInSerializer } from "../models/connectorIn";
import { type ConnectorOut, ConnectorOutSerializer } from "../models/connectorOut";
import { type ConnectorPatch, ConnectorPatchSerializer } from "../models/connectorPatch";
import type { ConnectorProduct } from "../models/connectorProduct";
import {
  type ConnectorUpdate,
  ConnectorUpdateSerializer,
} from "../models/connectorUpdate";
import {
  type ListResponseConnectorOut,
  ListResponseConnectorOutSerializer,
} from "../models/listResponseConnectorOut";
import type { Ordering } from "../models/ordering";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export interface ConnectorListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
  productType?: ConnectorProduct;
}

export interface ConnectorCreateOptions {
  idempotencyKey?: string;
}

export class Connector {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List all connectors for an application. */
  public list(options?: ConnectorListOptions): Promise<ListResponseConnectorOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/connector");

    request.setQueryParams({
      limit: options?.limit,
      iterator: options?.iterator,
      order: options?.order,
      product_type: options?.productType,
    });

    return request.send(
      this.requestCtx,
      ListResponseConnectorOutSerializer._fromJsonObject
    );
  }

  /** Create a new connector. */
  public create(
    connectorIn: ConnectorIn,
    options?: ConnectorCreateOptions
  ): Promise<ConnectorOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/connector");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(ConnectorInSerializer._toJsonObject(connectorIn));

    return request.send(this.requestCtx, ConnectorOutSerializer._fromJsonObject);
  }

  /** Get a connector. */
  public get(connectorId: string): Promise<ConnectorOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/connector/{connector_id}");

    request.setPathParam("connector_id", connectorId);

    return request.send(this.requestCtx, ConnectorOutSerializer._fromJsonObject);
  }

  /** Update a connector. */
  public update(
    connectorId: string,
    connectorUpdate: ConnectorUpdate
  ): Promise<ConnectorOut> {
    const request = new SvixRequest(HttpMethod.PUT, "/api/v1/connector/{connector_id}");

    request.setPathParam("connector_id", connectorId);
    request.setBody(ConnectorUpdateSerializer._toJsonObject(connectorUpdate));

    return request.send(this.requestCtx, ConnectorOutSerializer._fromJsonObject);
  }

  /** Delete a connector. */
  public delete(connectorId: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/connector/{connector_id}"
    );

    request.setPathParam("connector_id", connectorId);

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Partially update a connector. */
  public patch(
    connectorId: string,
    connectorPatch: ConnectorPatch
  ): Promise<ConnectorOut> {
    const request = new SvixRequest(HttpMethod.PATCH, "/api/v1/connector/{connector_id}");

    request.setPathParam("connector_id", connectorId);
    request.setBody(ConnectorPatchSerializer._toJsonObject(connectorPatch));

    return request.send(this.requestCtx, ConnectorOutSerializer._fromJsonObject);
  }
}
