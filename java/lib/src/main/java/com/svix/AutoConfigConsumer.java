package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internalapi.EndpointAutoConfigDeprecated;
import com.svix.internalapi.EndpointAutoconfig;
import com.svix.internalapi.DestinationAutoconfig;
import com.svix.internalapi.MessagePollerv2;
import com.svix.internalapi.MessagePollerv2ConsumerCommitOptions;
import com.svix.internalapi.MessagePollerv2ConsumerPollOptions;
import com.svix.models.AutoConfigSinkType;
import com.svix.models.AutoConfigSinkTypeConfig;
import com.svix.models.DestinationIn;
import com.svix.models.DestinationInConfig;
import com.svix.models.DestinationOut;
import com.svix.models.DestinationOutConfig;
import com.svix.models.EndpointOut;
import com.svix.models.PollerV2CommitIn;
import com.svix.models.PollerV2PollOut;
import com.svix.models.SinkInCommon;
import com.svix.models.DestinationStatus;
import com.svix.models.SubscribeIn;
import java.io.IOException;
import java.util.ArrayList;

public final class AutoConfigConsumer {
  private final String appId;
  private String sinkId;
  private final String autoconfigId;
  private final SinkInCommon sinkIn;
  private final Svix svix;

  public AutoConfigConsumer(final String token, final SinkInCommon sinkIn)
      throws AutoConfig.InvalidTokenException {
    AutoConfig.DecodedTokenContent content = AutoConfig.decodeToken(token);
    this.appId = content.getAppId();
    this.sinkId = content.getEndpointId();
    this.autoconfigId = content.getAutoconfigId();
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
  public DestinationOut subscribe() throws IOException, ApiException {
    if (autoconfigId != null) {
      DestinationOut destination = new DestinationAutoconfig(svix.getHttpClient())
          .subscribe(appId, autoconfigId, sinkInCommonToPollingDestination(sinkIn));
      this.sinkId = destination.getId();
      return destination;
    }

    EndpointOut endpoint =
        new EndpointAutoConfigDeprecated(svix.getHttpClient()).update(appId, sinkId, new SubscribeIn()
            .sink(new AutoConfigSinkType(
                new AutoConfigSinkTypeConfig.Poller(sinkIn))));
    return destinationOutFromV1Endpoint(endpoint);
  }

  private String getSinkId() throws IOException, ApiException {
    if (sinkId != null) {
      // Already have the sink id from subscribe() or the v1 token
      return sinkId;
    }

    // Get the sink id from the autoconfig id (v2)
    String destId = new EndpointAutoconfig(svix.getHttpClient()).get(appId, autoconfigId).getDestId();
    if (destId == null || destId.isEmpty()) {
      throw new IllegalStateException(
          "autoconfig subscription is pending. Have you called subscribe()?");
    }
    return destId;
  }

  public PollerV2PollOut receive(final String consumerId,
      final MessagePollerv2ConsumerPollOptions options) throws IOException, ApiException {
    String resolvedSinkId = getSinkId();
    return new MessagePollerv2(svix.getHttpClient()).consumerPoll(appId, resolvedSinkId, consumerId,
        options);
  }

  public PollerV2PollOut receive(final String consumerId) throws IOException, ApiException {
    return receive(consumerId, new MessagePollerv2ConsumerPollOptions());
  }

  public void commit(final String consumerId, final long offset,
      final MessagePollerv2ConsumerCommitOptions options) throws IOException, ApiException {
    String resolvedSinkId = getSinkId();
    PollerV2CommitIn commitIn = new PollerV2CommitIn();
    commitIn.setOffset(offset);
    new MessagePollerv2(svix.getHttpClient()).consumerCommit(appId, resolvedSinkId, consumerId,
        commitIn, options);
  }

  public void commit(final String consumerId, final long offset) throws IOException, ApiException {
    commit(consumerId, offset, new MessagePollerv2ConsumerCommitOptions());
  }

  private static DestinationOut destinationOutFromV1Endpoint(final EndpointOut endpoint) {
    DestinationOut destination = new DestinationOut();
    destination.setId(endpoint.getId());
    destination.setUid(endpoint.getUid());
    destination.setStatus(
        Boolean.TRUE.equals(endpoint.getDisabled()) ? DestinationStatus.DISABLED : DestinationStatus.ENABLED);
    destination.setCurrentIterator("");
    destination.setCreatedAt(endpoint.getCreatedAt());
    destination.setUpdatedAt(endpoint.getUpdatedAt());
    destination.setBatchSize(0);
    destination.setMaxWaitSecs(0);
    if (endpoint.getEventTypes() != null) {
      destination.setEventTypes(new ArrayList<>(endpoint.getEventTypes()));
    }
    if (endpoint.getChannels() != null) {
      destination.setChannels(new ArrayList<>(endpoint.getChannels()));
    }
    destination.setMetadata(endpoint.getMetadata());
    destination.setConfig(new DestinationOutConfig.PollingEndpoint());
    return destination;
  }

  private static DestinationIn sinkInCommonToPollingDestination(final SinkInCommon sink) {
    DestinationIn destination = new DestinationIn()
        .config(new DestinationInConfig.PollingEndpoint())
        .uid(sink.getUid())
        .metadata(sink.getMetadata());
    if (sink.getEventTypes() != null) {
      destination.eventTypes(new ArrayList<>(sink.getEventTypes()));
    }
    if (sink.getChannels() != null) {
      destination.channels(new ArrayList<>(sink.getChannels()));
    }
    return destination;
  }
}
