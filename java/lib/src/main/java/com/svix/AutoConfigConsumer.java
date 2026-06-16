package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internalapi.EndpointAutoConfig;
import com.svix.internalapi.MessagePollerv2;
import com.svix.internalapi.MessagePollerv2ConsumerCommitOptions;
import com.svix.internalapi.MessagePollerv2ConsumerPollOptions;
import com.svix.models.AutoConfigSinkType;
import com.svix.models.AutoConfigSinkTypeConfig;
import com.svix.models.EndpointOut;
import com.svix.models.PollerV2CommitIn;
import com.svix.models.PollerV2PollOut;
import com.svix.models.SinkInCommon;
import com.svix.models.SubscribeIn;
import java.io.IOException;

public final class AutoConfigConsumer {
  private final String appId;
  private final String sinkId;
  private final SinkInCommon sinkIn;
  private final Svix svix;

  public AutoConfigConsumer(final String token, final SinkInCommon sinkIn)
      throws AutoConfig.InvalidTokenException {
    AutoConfig.DecodedTokenContent content = AutoConfig.decodeToken(token);
    this.appId = content.getAppId();
    this.sinkId = content.getEndpointId();
    this.sinkIn = sinkIn;

    try {
      SvixOptions options = new SvixOptions();
      options.setServerUrl(content.getServerUrl());
      this.svix = new Svix(content.getTokenPlaintext(), options);
    } catch (IllegalArgumentException e) {
      throw new AutoConfig.InvalidTokenException(e);
    }
  }

  /** Registers this polling sink with Svix using the auto-config API. */
  public EndpointOut subscribe() throws IOException, ApiException {
    return new EndpointAutoConfig(svix.getHttpClient()).update(appId, sinkId, new SubscribeIn()
        .sink(new AutoConfigSinkType(new AutoConfigSinkTypeConfig.Poller(sinkIn))));
  }

  public PollerV2PollOut receive(final String consumerId,
      final MessagePollerv2ConsumerPollOptions options) throws IOException, ApiException {
    return new MessagePollerv2(svix.getHttpClient()).consumerPoll(appId, sinkId, consumerId,
        options);
  }

  public PollerV2PollOut receive(final String consumerId) throws IOException, ApiException {
    return receive(consumerId, new MessagePollerv2ConsumerPollOptions());
  }

  public void commit(final String consumerId, final long offset,
      final MessagePollerv2ConsumerCommitOptions options) throws IOException, ApiException {
    PollerV2CommitIn commitIn = new PollerV2CommitIn();
    commitIn.setOffset(offset);
    new MessagePollerv2(svix.getHttpClient()).consumerCommit(appId, sinkId, consumerId, commitIn,
        options);
  }

  public void commit(final String consumerId, final long offset) throws IOException, ApiException {
    commit(consumerId, offset, new MessagePollerv2ConsumerCommitOptions());
  }
}
