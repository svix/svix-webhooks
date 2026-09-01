import { SvixInternal } from "./api_internal";
import { Destination as InternalDestination } from "./api_internal/destination";
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
import { SinkStatus } from "./models/sinkStatus";
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
      const destination = await new InternalDestination(
        this.requestCtx
      ).autoconfig.subscribe(
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

  public async receive(
    consumerId: string,
    options?: MessagePollerv2ConsumerPollOptions
  ): Promise<PollerV2PollOut> {
    this.sinkId = this.sinkId ?? (await this.subscribe()).id;
    return new InternalMessagePollerv2(this.requestCtx).consumerPoll(
      this.appId,
      this.sinkId,
      consumerId,
      options
    );
  }

  public async commit(
    consumerId: string,
    offset: number,
    options?: MessagePollerv2ConsumerCommitOptions
  ): Promise<void> {
    this.sinkId = this.sinkId ?? (await this.subscribe()).id;
    return new InternalMessagePollerv2(this.requestCtx).consumerCommit(
      this.appId,
      this.sinkId,
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
    status: endpoint.disabled ? SinkStatus.Disabled : SinkStatus.Enabled,
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
