import { SvixInternal } from "./api_internal";
import { AutoconfigSubscription } from "./api_internal/autoconfigSubscription";
import { Endpoint as InternalEndpoint } from "./api_internal/endpoint";
import {
  MessagePollerv2 as InternalMessagePollerv2,
  type MessagePollerv2ConsumerCommitOptions,
  type MessagePollerv2ConsumerPollOptions,
} from "./api_internal/messagePollerv2";
import { decodeAutoconfigToken } from "./autoconfig";
import type { DestinationIn } from "./models/destinationIn";
import type { DestinationOut } from "./models/destinationOut";
import type { EndpointOut } from "./models/endpointOut";
import type { PollerV2PollOut } from "./models/pollerV2PollOut";
import type { SinkInCommon } from "./models/sinkInCommon";
import { DestinationStatus } from "./models/destinationStatus";
import type { SvixRequestContext } from "./request";

export class AutoConfigConsumer {
  private readonly appId: string;
  private readonly sinkIn: SinkInCommon;
  private readonly requestCtx: SvixRequestContext;
  private sinkId?: string;
  private readonly autoconfigId?: string;

  public constructor(token: string, sinkIn: SinkInCommon) {
    const decoded = decodeAutoconfigToken(token);

    this.appId = decoded.content.aid;
    this.sinkIn = sinkIn;

    if (decoded.version === "v1") {
      this.sinkId = decoded.content.eid;
    } else {
      this.autoconfigId = decoded.content.sid;
    }

    const svix = new SvixInternal(decoded.content.tok, {
      serverUrl: decoded.content.surl,
    });
    this.requestCtx = svix.getRequestCtx();
  }

  public async subscribe(): Promise<DestinationOut> {
    const endpoint = new InternalEndpoint(this.requestCtx);
    if (this.autoconfigId != null) {
      // v2
      const destination = await new AutoconfigSubscription(
        this.requestCtx
      ).destination.subscribe(
        this.appId,
        this.autoconfigId,
        sinkInCommonToPollingDestination(this.sinkIn)
      );
      this.sinkId = destination.id;
      return destination;
    }

    // v1
    return destinationOutFromV1Endpoint(
      await endpoint.autoConfigDeprecated.update(this.appId, this.sinkId as string, {
        sink: {
          type: "poller",
          config: this.sinkIn,
        },
      })
    );
  }

  private async getSinkId(): Promise<string> {
    if (this.sinkId != null) {
      // Already have the sink id from subscribe() or the v1 token
      return this.sinkId;
    }

    // Get the sink id from the autoconfig id (v2)
    const destId = (
      await new AutoconfigSubscription(this.requestCtx).get(
        this.appId,
        this.autoconfigId as string
      )
    ).destId;
    if (destId == null) {
      throw new Error("autoconfig subscription is pending. Have you called subscribe()?");
    }
    return destId;
  }

  public async receive(
    consumerId: string,
    options?: MessagePollerv2ConsumerPollOptions
  ): Promise<PollerV2PollOut> {
    const sinkId = await this.getSinkId();
    return new InternalMessagePollerv2(this.requestCtx).consumerPoll(
      this.appId,
      sinkId,
      consumerId,
      options
    );
  }

  public async commit(
    consumerId: string,
    offset: number,
    options?: MessagePollerv2ConsumerCommitOptions
  ): Promise<void> {
    const sinkId = await this.getSinkId();
    return new InternalMessagePollerv2(this.requestCtx).consumerCommit(
      this.appId,
      sinkId,
      consumerId,
      {
        offset,
      },
      options
    );
  }
}

function sinkInCommonToPollingDestination(sink: SinkInCommon): DestinationIn {
  return {
    type: "pollingEndpoint",
    eventTypes: sink.eventTypes ?? undefined,
    channels: sink.channels ?? undefined,
    metadata: sink.metadata,
    uid: sink.uid,
  };
}

function destinationOutFromV1Endpoint(endpoint: EndpointOut): DestinationOut {
  return {
    id: endpoint.id,
    uid: endpoint.uid,
    status: endpoint.disabled ? DestinationStatus.Disabled : DestinationStatus.Enabled,
    currentIterator: "",
    createdAt: endpoint.createdAt,
    updatedAt: endpoint.updatedAt,
    batchSize: 0,
    maxWaitSecs: 0,
    eventTypes: endpoint.eventTypes ?? undefined,
    channels: endpoint.channels ?? undefined,
    metadata: endpoint.metadata,
    type: "pollingEndpoint",
    config: {},
  };
}
