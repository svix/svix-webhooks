// this file is @generated

import {
  DashboardAccessOut,
  DashboardAccessOutSerializer,
} from "../models/dashboardAccessOut";
import {
  IngestSourceConsumerPortalAccessIn,
  IngestSourceConsumerPortalAccessInSerializer,
} from "../models/ingestSourceConsumerPortalAccessIn";
import { IngestEndpoint } from "./ingestEndpoint";
import { IngestSource } from "./ingestSource";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface IngestDashboardOptions {
  idempotencyKey?: string;
}

export class Ingest {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  public get endpoint() {
    return new IngestEndpoint(this.requestCtx);
  }

  public get source() {
    return new IngestSource(this.requestCtx);
  }

  /** Get access to the Ingest Source Consumer Portal. */
  public dashboard(
    sourceId: string,
    ingestSourceConsumerPortalAccessIn: IngestSourceConsumerPortalAccessIn,
    options?: IngestDashboardOptions
  ): Promise<DashboardAccessOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/ingest/api/v1/source/{source_id}/dashboard"
    );

    request.setPathParam("source_id", sourceId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(
      IngestSourceConsumerPortalAccessInSerializer._toJsonObject(
        ingestSourceConsumerPortalAccessIn
      )
    );

    return request.send(this.requestCtx, DashboardAccessOutSerializer._fromJsonObject);
  }
}
