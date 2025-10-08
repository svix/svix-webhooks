// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class StreamEventsCreateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class StreamEventsGetOptions : SvixOptionsBase
    {
        public ulong? Limit { get; set; }
        public string? Iterator { get; set; }
        public DateTime? After { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?>
                {
                    { "limit", Limit },
                    { "iterator", Iterator },
                    { "after", After },
                }
            );
        }
    }

    public class StreamEvents(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Creates events on the Stream.
        /// </summary>
        public async Task<CreateStreamEventsOut> CreateAsync(
            string streamId,
            CreateStreamEventsIn createStreamEventsIn,
            StreamEventsCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            createStreamEventsIn =
                createStreamEventsIn
                ?? throw new ArgumentNullException(nameof(createStreamEventsIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<CreateStreamEventsOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/stream/{stream_id}/events",
                    pathParams: new Dictionary<string, string> { { "stream_id", streamId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: createStreamEventsIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Creates events on the Stream.
        /// </summary>
        public CreateStreamEventsOut Create(
            string streamId,
            CreateStreamEventsIn createStreamEventsIn,
            StreamEventsCreateOptions? options = null
        )
        {
            createStreamEventsIn =
                createStreamEventsIn
                ?? throw new ArgumentNullException(nameof(createStreamEventsIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<CreateStreamEventsOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/stream/{stream_id}/events",
                    pathParams: new Dictionary<string, string> { { "stream_id", streamId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: createStreamEventsIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Create)} failed");

                throw;
            }
        }

        /// <summary>
        /// Iterate over a stream of events.
        ///
        /// The sink must be of type `poller` to use the poller endpoint.
        /// </summary>
        public async Task<EventStreamOut> GetAsync(
            string streamId,
            string sinkId,
            StreamEventsGetOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<EventStreamOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}/events",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
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
                _client.Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Iterate over a stream of events.
        ///
        /// The sink must be of type `poller` to use the poller endpoint.
        /// </summary>
        public EventStreamOut Get(
            string streamId,
            string sinkId,
            StreamEventsGetOptions? options = null
        )
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<EventStreamOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}/events",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Get)} failed");

                throw;
            }
        }
    }
}
