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

        public async Task<PollerV2PollOut> ReceiveAsync(
            string consumerId,
            MessagePollerv2ConsumerPollOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            sinkId ??= (await SubscribeAsync(cancellationToken)).Id;
            return await new MessagePollerv2(client).ConsumerPollAsync(
                appId,
                sinkId,
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
            sinkId ??= Subscribe().Id;
            return new MessagePollerv2(client).ConsumerPoll(appId, sinkId, consumerId, options);
        }

        public async Task CommitAsync(
            string consumerId,
            ulong offset,
            MessagePollerv2ConsumerCommitOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            sinkId ??= (await SubscribeAsync(cancellationToken)).Id;
            await new MessagePollerv2(client).ConsumerCommitAsync(
                appId,
                sinkId,
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
            sinkId ??= Subscribe().Id;
            new MessagePollerv2(client).ConsumerCommit(
                appId,
                sinkId,
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
                Status = endpoint.Disabled == true ? SinkStatus.Disabled : SinkStatus.Enabled,
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
