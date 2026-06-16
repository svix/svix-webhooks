using Svix.ApiInternal;
using Svix.Models;

namespace Svix
{
    public class AutoConfigConsumer
    {
        private readonly string appId;
        private readonly string sinkId;
        private readonly SinkInCommon sinkIn;
        private readonly SvixClient client;

        public AutoConfigConsumer(string token, SinkInCommon sinkIn)
        {
            sinkIn = sinkIn ?? throw new ArgumentNullException(nameof(sinkIn));

            var content = AutoConfig.DecodeAutoConfigTokenV1(token);

            appId = content.AppId;
            sinkId = content.EndpointId;
            this.sinkIn = sinkIn;
            client = new SvixClient(
                content.TokenPlaintext,
                new SvixOptions(serverUrl: content.ServerUrl)
            );
        }

        public async Task<EndpointOut> SubscribeAsync(CancellationToken cancellationToken = default)
        {
            return await new EndpointAutoConfig(client).UpdateAsync(
                appId,
                sinkId,
                new SubscribeIn
                {
                    Sink = new AutoConfigSinkType
                    {
                        Config = AutoConfigSinkTypeConfig.Poller(sinkIn),
                    },
                },
                cancellationToken
            );
        }

        public EndpointOut Subscribe()
        {
            return new EndpointAutoConfig(client).Update(
                appId,
                sinkId,
                new SubscribeIn
                {
                    Sink = new AutoConfigSinkType
                    {
                        Config = AutoConfigSinkTypeConfig.Poller(sinkIn),
                    },
                }
            );
        }

        public async Task<PollerV2PollOut> ReceiveAsync(
            string consumerId,
            MessagePollerv2ConsumerPollOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
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
            return new MessagePollerv2(client).ConsumerPoll(appId, sinkId, consumerId, options);
        }

        public async Task CommitAsync(
            string consumerId,
            ulong offset,
            MessagePollerv2ConsumerCommitOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
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
            new MessagePollerv2(client).ConsumerCommit(
                appId,
                sinkId,
                consumerId,
                new PollerV2CommitIn { Offset = offset },
                options
            );
        }
    }
}
