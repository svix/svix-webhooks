// this file is @generated
import { MessageOut, MessageOutSerializer } from "../models/messageOut";
import { RotatedUrlOut, RotatedUrlOutSerializer } from "../models/rotatedUrlOut";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface InboundMsgOptions {
  /** The event type's name */
  eventType?: string;
  idempotencyKey?: string;
}

export interface InboundRotateUrlOptions {
  idempotencyKey?: string;
}

export class Inbound {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Handles a raw inbound webhook for the application. */
  public msg(
    appId: string,
    inboundToken: string,
    options?: InboundMsgOptions
  ): Promise<MessageOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/inbound/msg/{inbound_token}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("inbound_token", inboundToken);
    request.setQueryParam("event_type", options?.eventType);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);

    return request.send(this.requestCtx, MessageOutSerializer._fromJsonObject);
  }

  /**
   * Invalidates the previous inbound url (if one exists), producing a new inbound
   * URL for this app.
   */
  public rotateUrl(
    appId: string,
    options?: InboundRotateUrlOptions
  ): Promise<RotatedUrlOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/inbound/rotate-url"
    );

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);

    return request.send(this.requestCtx, RotatedUrlOutSerializer._fromJsonObject);
  }
}
