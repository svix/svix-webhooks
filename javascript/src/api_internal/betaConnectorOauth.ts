// this file is @generated
import {
  IncomingWebhookPayloadOut,
  IncomingWebhookPayloadOutSerializer,
} from "../models/incomingWebhookPayloadOut";
import { OAuthPayloadIn, OAuthPayloadInSerializer } from "../models/oAuthPayloadIn";
import { OAuthPayloadOut, OAuthPayloadOutSerializer } from "../models/oAuthPayloadOut";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface BetaConnectorOauthDiscordOptions {
  idempotencyKey?: string;
}

export interface BetaConnectorOauthHubspotOptions {
  idempotencyKey?: string;
}

export interface BetaConnectorOauthSlackOptions {
  idempotencyKey?: string;
}

export class BetaConnectorOauth {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Get Discord Incoming webhook URL. */
  public discord(
    oAuthPayloadIn: OAuthPayloadIn,
    options?: BetaConnectorOauthDiscordOptions
  ): Promise<IncomingWebhookPayloadOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/transformation-template/oauth/discord"
    );

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(OAuthPayloadInSerializer._toJsonObject(oAuthPayloadIn));

    return request.send(
      this.requestCtx,
      IncomingWebhookPayloadOutSerializer._fromJsonObject
    );
  }

  /** Get Hubspot access token using authorization code. */
  public hubspot(
    oAuthPayloadIn: OAuthPayloadIn,
    options?: BetaConnectorOauthHubspotOptions
  ): Promise<OAuthPayloadOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/transformation-template/oauth/hubspot"
    );

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(OAuthPayloadInSerializer._toJsonObject(oAuthPayloadIn));

    return request.send(this.requestCtx, OAuthPayloadOutSerializer._fromJsonObject);
  }

  /** Get Slack Incoming webhook URL. */
  public slack(
    oAuthPayloadIn: OAuthPayloadIn,
    options?: BetaConnectorOauthSlackOptions
  ): Promise<IncomingWebhookPayloadOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/transformation-template/oauth/slack"
    );

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(OAuthPayloadInSerializer._toJsonObject(oAuthPayloadIn));

    return request.send(
      this.requestCtx,
      IncomingWebhookPayloadOutSerializer._fromJsonObject
    );
  }
}
