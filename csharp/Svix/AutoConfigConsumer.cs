using Svix.ApiInternal;
using Svix.Models;

namespace Svix
{
    public class AutoConfigConsumer
    {
        private readonly string _appId;
        private readonly string _sinkId;
        private readonly SinkInCommon _sinkIn;
        private readonly SvixClient _client;

        public AutoConfigConsumer(string token, SinkInCommon sinkIn)
        {
            sinkIn = sinkIn ?? throw new ArgumentNullException(nameof(sinkIn));

            var content = AutoConfig.DecodeAutoConfigTokenV1(token);

            _appId = content.AppId;
            _sinkId = content.EndpointId;
            _sinkIn = sinkIn;
            _client = new SvixClient(
                content.TokenPlaintext,
                new SvixOptions(serverUrl: content.ServerUrl)
            );
        }

        public async Task<EndpointOut> SubscribeAsync(CancellationToken cancellationToken = default)
        {
            return await new EndpointAutoConfig(_client).UpdateAsync(
                _appId,
                _sinkId,
                new SubscribeIn
                {
                    Sink = new AutoConfigSinkType
                    {
                        Config = AutoConfigSinkTypeConfig.Poller(_sinkIn),
                    },
                },
                cancellationToken
            );
        }

        public EndpointOut Subscribe()
        {
            return new EndpointAutoConfig(_client).Update(
                _appId,
                _sinkId,
                new SubscribeIn
                {
                    Sink = new AutoConfigSinkType
                    {
                        Config = AutoConfigSinkTypeConfig.Poller(_sinkIn),
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
            return await new MessagePollerv2(_client).ConsumerPollAsync(
                _appId,
                _sinkId,
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
            return new MessagePollerv2(_client).ConsumerPoll(_appId, _sinkId, consumerId, options);
        }

        public async Task CommitAsync(
            string consumerId,
            ulong offset,
            MessagePollerv2ConsumerCommitOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            await new MessagePollerv2(_client).ConsumerCommitAsync(
                _appId,
                _sinkId,
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
            new MessagePollerv2(_client).ConsumerCommit(
                _appId,
                _sinkId,
                consumerId,
                new PollerV2CommitIn { Offset = offset },
                options
            );
        }
    }
}
