import { SvixInternal } from "./api_internal";
import { Endpoint as InternalEndpoint } from "./api_internal/endpoint";
import {
  MessagePollerv2 as InternalMessagePollerv2,
  type MessagePollerv2ConsumerCommitOptions,
  type MessagePollerv2ConsumerPollOptions,
} from "./api_internal/messagePollerv2";
import { decodeAutoconfigTokenV1 } from "./autoconfig";
import type { EndpointOut } from "./models/endpointOut";
import type { PollerV2PollOut } from "./models/pollerV2PollOut";
import type { SinkInCommon } from "./models/sinkInCommon";
import type { SvixRequestContext } from "./request";

export class AutoConfigConsumer {
  private readonly appId: string;
  private readonly sinkId: string;
  private readonly sinkIn: SinkInCommon;
  private readonly requestCtx: SvixRequestContext;

  public constructor(token: string, sinkIn: SinkInCommon) {
    const content = decodeAutoconfigTokenV1(token);

    this.appId = content.aid;
    this.sinkId = content.eid;
    this.sinkIn = sinkIn;

    const svix = new SvixInternal(content.tok, { serverUrl: content.surl });
    this.requestCtx = svix.getRequestCtx();
  }

  public subscribe(): Promise<EndpointOut> {
    return new InternalEndpoint(this.requestCtx).auto_config.update(
      this.appId,
      this.sinkId,
      {
        sink: {
          type: "poller",
          config: this.sinkIn,
        },
      }
    );
  }

  public receive(
    consumerId: string,
    options?: MessagePollerv2ConsumerPollOptions
  ): Promise<PollerV2PollOut> {
    return new InternalMessagePollerv2(this.requestCtx).consumerPoll(
      this.appId,
      this.sinkId,
      consumerId,
      options
    );
  }

  public commit(
    consumerId: string,
    offset: number,
    options?: MessagePollerv2ConsumerCommitOptions
  ): Promise<void> {
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
