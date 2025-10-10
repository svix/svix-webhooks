// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class StreamingEventTypeListOptions : SvixOptionsBase
    {
        public ulong? Limit { get; set; }
        public string? Iterator { get; set; }
        public Ordering? Order { get; set; }
        public bool? IncludeArchived { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?>
                {
                    { "limit", Limit },
                    { "iterator", Iterator },
                    { "order", Order },
                    { "include_archived", IncludeArchived },
                }
            );
        }
    }

    public class StreamingEventTypeCreateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class StreamingEventTypeDeleteOptions : SvixOptionsBase
    {
        public bool? Expunge { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(new Dictionary<string, object?> { { "expunge", Expunge } });
        }
    }

    public class StreamingEventType(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// List of all the organization's event types for streaming.
        /// </summary>
        public async Task<ListResponseStreamEventTypeOut> ListAsync(
            StreamingEventTypeListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<ListResponseStreamEventTypeOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/stream/event-type",
                        queryParams: options?.QueryParams(),
                        headerParams: options?.HeaderParams(),
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// List of all the organization's event types for streaming.
        /// </summary>
        public ListResponseStreamEventTypeOut List(StreamingEventTypeListOptions? options = null)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ListResponseStreamEventTypeOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream/event-type",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(List)} failed");

                throw;
            }
        }

        /// <summary>
        /// Create an event type for Streams.
        /// </summary>
        public async Task<StreamEventTypeOut> CreateAsync(
            StreamEventTypeIn streamEventTypeIn,
            StreamingEventTypeCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            streamEventTypeIn =
                streamEventTypeIn ?? throw new ArgumentNullException(nameof(streamEventTypeIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<StreamEventTypeOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/stream/event-type",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: streamEventTypeIn,
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
        /// Create an event type for Streams.
        /// </summary>
        public StreamEventTypeOut Create(
            StreamEventTypeIn streamEventTypeIn,
            StreamingEventTypeCreateOptions? options = null
        )
        {
            streamEventTypeIn =
                streamEventTypeIn ?? throw new ArgumentNullException(nameof(streamEventTypeIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<StreamEventTypeOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/stream/event-type",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: streamEventTypeIn
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
        /// Get an event type.
        /// </summary>
        public async Task<StreamEventTypeOut> GetAsync(
            string name,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<StreamEventTypeOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream/event-type/{name}",
                    pathParams: new Dictionary<string, string> { { "name", name } },
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
        /// Get an event type.
        /// </summary>
        public StreamEventTypeOut Get(string name)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<StreamEventTypeOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream/event-type/{name}",
                    pathParams: new Dictionary<string, string> { { "name", name } }
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Get)} failed");

                throw;
            }
        }

        /// <summary>
        /// Update or create a event type for Streams.
        /// </summary>
        public async Task<StreamEventTypeOut> UpdateAsync(
            string name,
            StreamEventTypeIn streamEventTypeIn,
            CancellationToken cancellationToken = default
        )
        {
            streamEventTypeIn =
                streamEventTypeIn ?? throw new ArgumentNullException(nameof(streamEventTypeIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<StreamEventTypeOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/stream/event-type/{name}",
                    pathParams: new Dictionary<string, string> { { "name", name } },
                    content: streamEventTypeIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(UpdateAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Update or create a event type for Streams.
        /// </summary>
        public StreamEventTypeOut Update(string name, StreamEventTypeIn streamEventTypeIn)
        {
            streamEventTypeIn =
                streamEventTypeIn ?? throw new ArgumentNullException(nameof(streamEventTypeIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<StreamEventTypeOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/stream/event-type/{name}",
                    pathParams: new Dictionary<string, string> { { "name", name } },
                    content: streamEventTypeIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Update)} failed");

                throw;
            }
        }

        /// <summary>
        /// Delete an event type.
        /// </summary>
        public async Task<bool> DeleteAsync(
            string name,
            StreamingEventTypeDeleteOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/stream/event-type/{name}",
                    pathParams: new Dictionary<string, string> { { "name", name } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(DeleteAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Delete an event type.
        /// </summary>
        public bool Delete(string name, StreamingEventTypeDeleteOptions? options = null)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/stream/event-type/{name}",
                    pathParams: new Dictionary<string, string> { { "name", name } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Delete)} failed");

                throw;
            }
        }

        /// <summary>
        /// Patch an event type for Streams.
        /// </summary>
        public async Task<StreamEventTypeOut> PatchAsync(
            string name,
            StreamEventTypePatch streamEventTypePatch,
            CancellationToken cancellationToken = default
        )
        {
            streamEventTypePatch =
                streamEventTypePatch
                ?? throw new ArgumentNullException(nameof(streamEventTypePatch));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<StreamEventTypeOut>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/stream/event-type/{name}",
                    pathParams: new Dictionary<string, string> { { "name", name } },
                    content: streamEventTypePatch,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(PatchAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Patch an event type for Streams.
        /// </summary>
        public StreamEventTypeOut Patch(string name, StreamEventTypePatch streamEventTypePatch)
        {
            streamEventTypePatch =
                streamEventTypePatch
                ?? throw new ArgumentNullException(nameof(streamEventTypePatch));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<StreamEventTypeOut>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/stream/event-type/{name}",
                    pathParams: new Dictionary<string, string> { { "name", name } },
                    content: streamEventTypePatch
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Patch)} failed");

                throw;
            }
        }
    }
}
