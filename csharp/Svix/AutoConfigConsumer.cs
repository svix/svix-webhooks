using Svix.ApiInternal;
using Svix.Models;

namespace Svix
{
    public class AutoConfigConsumer
    {
        private readonly string appId;
        private string? sinkId;
        private readonly string? autoconfigId;
        private readonly SinkInCommon sinkIn;
        private readonly SvixClient client;

        public AutoConfigConsumer(string token, SinkInCommon sinkIn)
        {
            sinkIn = sinkIn ?? throw new ArgumentNullException(nameof(sinkIn));

            var content = AutoConfig.DecodeAutoConfigToken(token);

            appId = content.AppId;
            sinkId = content.EndpointId;
            autoconfigId = content.AutoconfigId;
            this.sinkIn = sinkIn;
            client = new SvixClient(
                content.TokenPlaintext,
                new SvixOptions(serverUrl: content.ServerUrl)
            );
        }

        public async Task<DestinationOut> SubscribeAsync(
            CancellationToken cancellationToken = default
        )
        {
            if (autoconfigId != null)
            {
                var destination = await new DestinationAutoconfig(client).SubscribeAsync(
                    appId,
                    autoconfigId,
                    SinkInCommonToPollingDestination(sinkIn),
                    cancellationToken
                );
                sinkId = destination.Id;
                return destination;
            }

            var endpoint = await new EndpointAutoConfigDeprecated(client).UpdateAsync(
                appId,
                sinkId!,
                new SubscribeIn
                {
                    Sink = new AutoConfigSinkType
                    {
                        Config = AutoConfigSinkTypeConfig.Poller(sinkIn),
                    },
                },
                cancellationToken
            );
            return DestinationOutFromV1Endpoint(endpoint);
        }

        public DestinationOut Subscribe()
        {
            if (autoconfigId != null)
            {
                var destination = new DestinationAutoconfig(client).Subscribe(
                    appId,
                    autoconfigId,
                    SinkInCommonToPollingDestination(sinkIn)
                );
                sinkId = destination.Id;
                return destination;
            }

            var endpoint = new EndpointAutoConfigDeprecated(client).Update(
                appId,
                sinkId!,
                new SubscribeIn
                {
                    Sink = new AutoConfigSinkType
                    {
                        Config = AutoConfigSinkTypeConfig.Poller(sinkIn),
                    },
                }
            );
            return DestinationOutFromV1Endpoint(endpoint);
        }

        private async Task<string> GetSinkIdAsync(CancellationToken cancellationToken = default)
        {
            if (sinkId != null)
            {
                // Already have the sink id from subscribe() or the v1 token
                return sinkId;
            }

            // Get the sink id from the autoconfig id (v2)
            var subscription = await new EndpointAutoconfig(client).GetAsync(
                appId,
                autoconfigId!,
                cancellationToken
            );
            return subscription.DestId
                ?? throw new InvalidOperationException(
                    "autoconfig subscription is pending. Have you called subscribe()?"
                );
        }

        private string GetSinkId()
        {
            if (sinkId != null)
            {
                // Already have the sink id from subscribe() or the v1 token
                return sinkId;
            }

            // Get the sink id from the autoconfig id (v2)
            return new EndpointAutoconfig(client).Get(appId, autoconfigId!).DestId
                ?? throw new InvalidOperationException(
                    "autoconfig subscription is pending. Have you called subscribe()?"
                );
        }

        public async Task<PollerV2PollOut> ReceiveAsync(
            string consumerId,
            MessagePollerv2ConsumerPollOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            var resolvedSinkId = await GetSinkIdAsync(cancellationToken);
            return await new MessagePollerv2(client).ConsumerPollAsync(
                appId,
                resolvedSinkId,
                consumerId,
                options,
                cancellationToken
            );
        }

        public PollerV2PollOut Receive(
            string consumerId,
            MessagePollerv2ConsumerPollOptions? options = null
        )
        {
            return new MessagePollerv2(client).ConsumerPoll(
                appId,
                GetSinkId(),
                consumerId,
                options
            );
        }

        public async Task CommitAsync(
            string consumerId,
            ulong offset,
            MessagePollerv2ConsumerCommitOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            var resolvedSinkId = await GetSinkIdAsync(cancellationToken);
            await new MessagePollerv2(client).ConsumerCommitAsync(
                appId,
                resolvedSinkId,
                consumerId,
                new PollerV2CommitIn { Offset = offset },
                options,
                cancellationToken
            );
        }

        public void Commit(
            string consumerId,
            ulong offset,
            MessagePollerv2ConsumerCommitOptions? options = null
        )
        {
            new MessagePollerv2(client).ConsumerCommit(
                appId,
                GetSinkId(),
                consumerId,
                new PollerV2CommitIn { Offset = offset },
                options
            );
        }

        private static DestinationIn SinkInCommonToPollingDestination(SinkInCommon sink)
        {
            return new DestinationIn
            {
                Uid = sink.Uid,
                EventTypes = sink.EventTypes,
                Channels = sink.Channels,
                Metadata = sink.Metadata,
                Config = DestinationInConfig.PollingEndpoint(),
            };
        }

        private static DestinationOut DestinationOutFromV1Endpoint(EndpointOut endpoint)
        {
            return new DestinationOut
            {
                Id = endpoint.Id,
                Uid = endpoint.Uid,
                Status =
                    endpoint.Disabled == true
                        ? DestinationStatus.Disabled
                        : DestinationStatus.Enabled,
                CurrentIterator = "",
                CreatedAt = endpoint.CreatedAt,
                UpdatedAt = endpoint.UpdatedAt,
                BatchSize = 0,
                MaxWaitSecs = 0,
                EventTypes = endpoint.EventTypes,
                Channels = endpoint.Channels,
                Metadata = endpoint.Metadata,
                Config = DestinationOutConfig.PollingEndpoint(),
            };
        }
    }
}
