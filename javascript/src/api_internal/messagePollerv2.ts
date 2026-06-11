// this file is @generated

import {
  type PollerV2CommitIn,
  PollerV2CommitInSerializer,
} from "../models/pollerV2CommitIn";
import {
  type PollerV2PollOut,
  PollerV2PollOutSerializer,
} from "../models/pollerV2PollOut";
import type { StartingPosition } from "../models/startingPosition";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export interface MessagePollerv2ConsumerPollOptions {
  limit?: number;
  /** Lease duration in milliseconds. */
  leaseDurationMs?: number;
  startingPosition?: StartingPosition;
}

export interface MessagePollerv2ConsumerCommitOptions {
  idempotencyKey?: string;
}

export class MessagePollerv2 {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Poll messages from a sink. */
  public async consumerPoll(
    appId: string,
    sinkId: string,
    consumerId: string,
    options?: MessagePollerv2ConsumerPollOptions
  ): Promise<PollerV2PollOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/polling-endpoint/{sink_id}/consumer/{consumer_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("sink_id", sinkId);
    request.setPathParam("consumer_id", consumerId);
    request.setQueryParams({
      limit: options?.limit,
      lease_duration_ms: options?.leaseDurationMs,
      starting_position: options?.startingPosition,
    });

    return await request.send(this.requestCtx, PollerV2PollOutSerializer._fromJsonObject);
  }

  /** Ack a message offset for a sink's consumer. */
  public async consumerCommit(
    appId: string,
    sinkId: string,
    consumerId: string,
    pollerV2CommitIn: PollerV2CommitIn,
    options?: MessagePollerv2ConsumerCommitOptions
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/polling-endpoint/{sink_id}/consumer/{consumer_id}/commit"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("sink_id", sinkId);
    request.setPathParam("consumer_id", consumerId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(PollerV2CommitInSerializer._toJsonObject(pollerV2CommitIn));

    return await request.sendNoResponseBody(this.requestCtx);
  }
}
