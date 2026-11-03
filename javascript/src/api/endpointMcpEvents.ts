// this file is @generated

import {
  type McpEventsSubscribeIn,
  McpEventsSubscribeInSerializer,
} from "../models/mcpEventsSubscribeIn";
import {
  type McpEventsSubscribeOut,
  McpEventsSubscribeOutSerializer,
} from "../models/mcpEventsSubscribeOut";
import {
  type McpEventsUnsubscribeIn,
  McpEventsUnsubscribeInSerializer,
} from "../models/mcpEventsUnsubscribeIn";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export interface EndpointMcpEventsSubscribeOptions {
  idempotencyKey?: string;
}

export interface EndpointMcpEventsUnsubscribeOptions {
  idempotencyKey?: string;
}

export class EndpointMcpEvents {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Create or refresh an MCP Events subscription. */
  public async subscribe(
    appId: string,
    mcpEventsSubscribeIn: McpEventsSubscribeIn,
    options?: EndpointMcpEventsSubscribeOptions
  ): Promise<McpEventsSubscribeOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/mcp-events/subscribe"
    );

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(McpEventsSubscribeInSerializer._toJsonObject(mcpEventsSubscribeIn));

    return await request.send(
      this.requestCtx,
      McpEventsSubscribeOutSerializer._fromJsonObject
    );
  }

  /**
   * Stop an MCP Events subscription.
   *
   * Forward an MCP server's `events/unsubscribe` requests here. The
   * subscription's endpoint is deleted. Unsubscribing from a subscription that
   * doesn't exist succeeds, so this can be repeated.
   */
  public async unsubscribe(
    appId: string,
    mcpEventsUnsubscribeIn: McpEventsUnsubscribeIn,
    options?: EndpointMcpEventsUnsubscribeOptions
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/mcp-events/unsubscribe"
    );

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(
      McpEventsUnsubscribeInSerializer._toJsonObject(mcpEventsUnsubscribeIn)
    );

    return await request.sendNoResponseBody(this.requestCtx);
  }
}
