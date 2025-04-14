// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class MessagePollerPollOptions : SvixOptionsBase
    {
        public ulong? Limit { get; set; }
        public string? Iterator { get; set; }
        public string? EventType { get; set; }
        public string? Channel { get; set; }
        public DateTime? After { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?>
                {
                    { "limit", Limit },
                    { "iterator", Iterator },
                    { "event_type", EventType },
                    { "channel", Channel },
                    { "after", After },
                }
            );
        }
    }

    public class MessagePollerConsumerPollOptions : SvixOptionsBase
    {
        public ulong? Limit { get; set; }
        public string? Iterator { get; set; }
        public string? EventType { get; set; }
        public string? Channel { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?>
                {
                    { "limit", Limit },
                    { "iterator", Iterator },
                    { "event_type", EventType },
                    { "channel", Channel },
                }
            );
        }
    }

    public class MessagePollerConsumerSeekOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class MessagePoller(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Reads the stream of created messages for an application, filtered on the Sink's event types and Channels.
        /// </summary>
        public async Task<PollingEndpointOut> PollAsync(
            string appId,
            string sinkId,
            MessagePollerPollOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<PollingEndpointOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/poller/{sink_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "sink_id", sinkId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(PollAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Reads the stream of created messages for an application, filtered on the Sink's event types and Channels.
        /// </summary>
        public PollingEndpointOut Poll(
            string appId,
            string sinkId,
            MessagePollerPollOptions? options = null
        )
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<PollingEndpointOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/poller/{sink_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "sink_id", sinkId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Poll)} failed");

                throw;
            }
        }

        /// <summary>
        /// Reads the stream of created messages for an application, filtered on the Sink's event types and
        /// Channels, using server-managed iterator tracking.
        /// </summary>
        public async Task<PollingEndpointOut> ConsumerPollAsync(
            string appId,
            string sinkId,
            string consumerId,
            MessagePollerConsumerPollOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<PollingEndpointOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}",
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
        /// Reads the stream of created messages for an application, filtered on the Sink's event types and
        /// Channels, using server-managed iterator tracking.
        /// </summary>
        public PollingEndpointOut ConsumerPoll(
            string appId,
            string sinkId,
            string consumerId,
            MessagePollerConsumerPollOptions? options = null
        )
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<PollingEndpointOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}",
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
        /// Sets the starting offset for the consumer of a polling endpoint.
        /// </summary>
        public async Task<PollingEndpointConsumerSeekOut> ConsumerSeekAsync(
            string appId,
            string sinkId,
            string consumerId,
            PollingEndpointConsumerSeekIn pollingEndpointConsumerSeekIn,
            MessagePollerConsumerSeekOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            pollingEndpointConsumerSeekIn =
                pollingEndpointConsumerSeekIn
                ?? throw new ArgumentNullException(nameof(pollingEndpointConsumerSeekIn));
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<PollingEndpointConsumerSeekOut>(
                        method: HttpMethod.Post,
                        path: "/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}/seek",
                        pathParams: new Dictionary<string, string>
                        {
                            { "app_id", appId },
                            { "sink_id", sinkId },
                            { "consumer_id", consumerId },
                        },
                        queryParams: options?.QueryParams(),
                        headerParams: options?.HeaderParams(),
                        content: pollingEndpointConsumerSeekIn,
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ConsumerSeekAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Sets the starting offset for the consumer of a polling endpoint.
        /// </summary>
        public PollingEndpointConsumerSeekOut ConsumerSeek(
            string appId,
            string sinkId,
            string consumerId,
            PollingEndpointConsumerSeekIn pollingEndpointConsumerSeekIn,
            MessagePollerConsumerSeekOptions? options = null
        )
        {
            pollingEndpointConsumerSeekIn =
                pollingEndpointConsumerSeekIn
                ?? throw new ArgumentNullException(nameof(pollingEndpointConsumerSeekIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<PollingEndpointConsumerSeekOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}/seek",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "sink_id", sinkId },
                        { "consumer_id", consumerId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: pollingEndpointConsumerSeekIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ConsumerSeek)} failed");

                throw;
            }
        }
    }
}
