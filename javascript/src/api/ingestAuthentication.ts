// this file is @generated

import {
  type AppPortalAccessOut,
  AppPortalAccessOutSerializer,
} from "../models/appPortalAccessOut";
import {
  type IngestSourceConsumerPortalAccessIn,
  IngestSourceConsumerPortalAccessInSerializer,
} from "../models/ingestSourceConsumerPortalAccessIn";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export interface IngestAuthenticationConsumerPortalAccessOptions {
  idempotencyKey?: string;
}

export class IngestAuthentication {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Get access to the Ingest Source Consumer Portal. */
  public async consumerPortalAccess(
    sourceId: string,
    ingestSourceConsumerPortalAccessIn: IngestSourceConsumerPortalAccessIn = {},
    options?: IngestAuthenticationConsumerPortalAccessOptions
  ): Promise<AppPortalAccessOut> {
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

    return await request.send(
      this.requestCtx,
      AppPortalAccessOutSerializer._fromJsonObject
    );
  }
}
