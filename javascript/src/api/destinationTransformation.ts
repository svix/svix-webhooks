// this file is @generated

import {
  type DestinationTransformIn,
  DestinationTransformInSerializer,
} from "../models/destinationTransformIn";
import {
  type DestinationTransformationOut,
  DestinationTransformationOutSerializer,
} from "../models/destinationTransformationOut";
import { type EmptyResponse, EmptyResponseSerializer } from "../models/emptyResponse";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export class DestinationTransformation {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Get the transformation code associated with this destination. */
  public async get(
    appId: string,
    destinationId: string
  ): Promise<DestinationTransformationOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/destination/{destination_id}/transformation"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("destination_id", destinationId);

    return await request.send(
      this.requestCtx,
      DestinationTransformationOutSerializer._fromJsonObject
    );
  }

  /** Set or unset the transformation code associated with this destination. */
  public async patch(
    appId: string,
    destinationId: string,
    destinationTransformIn: DestinationTransformIn = {}
  ): Promise<EmptyResponse> {
    const request = new SvixRequest(
      HttpMethod.PATCH,
      "/api/v1/app/{app_id}/destination/{destination_id}/transformation"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("destination_id", destinationId);
    request.setBody(
      DestinationTransformInSerializer._toJsonObject(destinationTransformIn)
    );

    return await request.send(this.requestCtx, EmptyResponseSerializer._fromJsonObject);
  }
}
