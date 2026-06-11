// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix.ApiInternal
{
    public class MessagePollerv2ConsumerPollOptions : SvixOptionsBase
    {
        public ulong? Limit { get; set; }
        public ulong? LeaseDurationMs { get; set; }
        public StartingPosition? StartingPosition { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?>
                {
                    { "limit", Limit },
                    { "lease_duration_ms", LeaseDurationMs },
                    { "starting_position", StartingPosition },
                }
            );
        }
    }

    public class MessagePollerv2ConsumerCommitOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class MessagePollerv2(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Poll messages from a sink.
        /// </summary>
        public async Task<PollerV2PollOut> ConsumerPollAsync(
            string appId,
            string sinkId,
            string consumerId,
            MessagePollerv2ConsumerPollOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<PollerV2PollOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/polling-endpoint/{sink_id}/consumer/{consumer_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "sink_id", sinkId },
                        { "consumer_id", consumerId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ConsumerPollAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Poll messages from a sink.
        /// </summary>
        public PollerV2PollOut ConsumerPoll(
            string appId,
            string sinkId,
            string consumerId,
            MessagePollerv2ConsumerPollOptions? options = null
        )
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<PollerV2PollOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/polling-endpoint/{sink_id}/consumer/{consumer_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "sink_id", sinkId },
                        { "consumer_id", consumerId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ConsumerPoll)} failed");

                throw;
            }
        }

        /// <summary>
        /// Ack a message offset for a sink's consumer.
        /// </summary>
        public async Task<bool> ConsumerCommitAsync(
            string appId,
            string sinkId,
            string consumerId,
            PollerV2CommitIn pollerV2CommitIn,
            MessagePollerv2ConsumerCommitOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            pollerV2CommitIn =
                pollerV2CommitIn ?? throw new ArgumentNullException(nameof(pollerV2CommitIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/polling-endpoint/{sink_id}/consumer/{consumer_id}/commit",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "sink_id", sinkId },
                        { "consumer_id", consumerId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: pollerV2CommitIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ConsumerCommitAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Ack a message offset for a sink's consumer.
        /// </summary>
        public bool ConsumerCommit(
            string appId,
            string sinkId,
            string consumerId,
            PollerV2CommitIn pollerV2CommitIn,
            MessagePollerv2ConsumerCommitOptions? options = null
        )
        {
            pollerV2CommitIn =
                pollerV2CommitIn ?? throw new ArgumentNullException(nameof(pollerV2CommitIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/polling-endpoint/{sink_id}/consumer/{consumer_id}/commit",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "sink_id", sinkId },
                        { "consumer_id", consumerId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: pollerV2CommitIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ConsumerCommit)} failed");

                throw;
            }
        }
    }
}
