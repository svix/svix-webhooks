// this file is @generated
import {
  MessageSubscriberAuthTokenOut,
  MessageSubscriberAuthTokenOutSerializer,
} from "../models/messageSubscriberAuthTokenOut";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface MessageEventsSubscriptionCreateTokenOptions {
  idempotencyKey?: string;
}

export class MessageEventsSubscription {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Creates an auth token that can be used with the `v1.message.events-subscription` endpoint. */
  public createToken(
    appId: string,
    subscriptionId: string,
    options?: MessageEventsSubscriptionCreateTokenOptions
  ): Promise<MessageSubscriberAuthTokenOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/events/subscription/{subscription_id}/create-token"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("subscription_id", subscriptionId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);

    return request.send(
      this.requestCtx,
      MessageSubscriberAuthTokenOutSerializer._fromJsonObject
    );
  }
}
