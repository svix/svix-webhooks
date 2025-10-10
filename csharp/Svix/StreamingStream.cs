// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class StreamingStreamListOptions : SvixOptionsBase
    {
        public ulong? Limit { get; set; }
        public string? Iterator { get; set; }
        public Ordering? Order { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?>
                {
                    { "limit", Limit },
                    { "iterator", Iterator },
                    { "order", Order },
                }
            );
        }
    }

    public class StreamingStreamCreateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class StreamingStream(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// List of all the organization's streams.
        /// </summary>
        public async Task<ListResponseStreamOut> ListAsync(
            StreamingStreamListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<ListResponseStreamOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream",
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
        /// List of all the organization's streams.
        /// </summary>
        public ListResponseStreamOut List(StreamingStreamListOptions? options = null)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ListResponseStreamOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream",
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
        /// Creates a new stream.
        /// </summary>
        public async Task<StreamOut> CreateAsync(
            StreamIn streamIn,
            StreamingStreamCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            streamIn = streamIn ?? throw new ArgumentNullException(nameof(streamIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<StreamOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/stream",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: streamIn,
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
        /// Creates a new stream.
        /// </summary>
        public StreamOut Create(StreamIn streamIn, StreamingStreamCreateOptions? options = null)
        {
            streamIn = streamIn ?? throw new ArgumentNullException(nameof(streamIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<StreamOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/stream",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: streamIn
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
        /// Get a stream by id or uid.
        /// </summary>
        public async Task<StreamOut> GetAsync(
            string streamId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<StreamOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream/{stream_id}",
                    pathParams: new Dictionary<string, string> { { "stream_id", streamId } },
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
        /// Get a stream by id or uid.
        /// </summary>
        public StreamOut Get(string streamId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<StreamOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream/{stream_id}",
                    pathParams: new Dictionary<string, string> { { "stream_id", streamId } }
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
        /// Update a stream.
        /// </summary>
        public async Task<StreamOut> UpdateAsync(
            string streamId,
            StreamIn streamIn,
            CancellationToken cancellationToken = default
        )
        {
            streamIn = streamIn ?? throw new ArgumentNullException(nameof(streamIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<StreamOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/stream/{stream_id}",
                    pathParams: new Dictionary<string, string> { { "stream_id", streamId } },
                    content: streamIn,
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
        /// Update a stream.
        /// </summary>
        public StreamOut Update(string streamId, StreamIn streamIn)
        {
            streamIn = streamIn ?? throw new ArgumentNullException(nameof(streamIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<StreamOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/stream/{stream_id}",
                    pathParams: new Dictionary<string, string> { { "stream_id", streamId } },
                    content: streamIn
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
        /// Delete a stream.
        /// </summary>
        public async Task<bool> DeleteAsync(
            string streamId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/stream/{stream_id}",
                    pathParams: new Dictionary<string, string> { { "stream_id", streamId } },
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
        /// Delete a stream.
        /// </summary>
        public bool Delete(string streamId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/stream/{stream_id}",
                    pathParams: new Dictionary<string, string> { { "stream_id", streamId } }
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
        /// Partially update a stream.
        /// </summary>
        public async Task<StreamOut> PatchAsync(
            string streamId,
            StreamPatch streamPatch,
            CancellationToken cancellationToken = default
        )
        {
            streamPatch = streamPatch ?? throw new ArgumentNullException(nameof(streamPatch));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<StreamOut>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/stream/{stream_id}",
                    pathParams: new Dictionary<string, string> { { "stream_id", streamId } },
                    content: streamPatch,
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
        /// Partially update a stream.
        /// </summary>
        public StreamOut Patch(string streamId, StreamPatch streamPatch)
        {
            streamPatch = streamPatch ?? throw new ArgumentNullException(nameof(streamPatch));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<StreamOut>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/stream/{stream_id}",
                    pathParams: new Dictionary<string, string> { { "stream_id", streamId } },
                    content: streamPatch
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
