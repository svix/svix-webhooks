// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class StreamingSinkListOptions : SvixOptionsBase
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

    public class StreamingSinkCreateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class StreamingSinkRotateSecretOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class StreamingSink(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// List of all the stream's sinks.
        /// </summary>
        public async Task<ListResponseStreamSinkOut> ListAsync(
            string streamId,
            StreamingSinkListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<ListResponseStreamSinkOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/stream/{stream_id}/sink",
                        pathParams: new Dictionary<string, string> { { "stream_id", streamId } },
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
        /// List of all the stream's sinks.
        /// </summary>
        public ListResponseStreamSinkOut List(
            string streamId,
            StreamingSinkListOptions? options = null
        )
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ListResponseStreamSinkOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream/{stream_id}/sink",
                    pathParams: new Dictionary<string, string> { { "stream_id", streamId } },
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
        /// Creates a new sink.
        /// </summary>
        public async Task<StreamSinkOut> CreateAsync(
            string streamId,
            StreamSinkIn streamSinkIn,
            StreamingSinkCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            streamSinkIn = streamSinkIn ?? throw new ArgumentNullException(nameof(streamSinkIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<StreamSinkOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/stream/{stream_id}/sink",
                    pathParams: new Dictionary<string, string> { { "stream_id", streamId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: streamSinkIn,
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
        /// Creates a new sink.
        /// </summary>
        public StreamSinkOut Create(
            string streamId,
            StreamSinkIn streamSinkIn,
            StreamingSinkCreateOptions? options = null
        )
        {
            streamSinkIn = streamSinkIn ?? throw new ArgumentNullException(nameof(streamSinkIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<StreamSinkOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/stream/{stream_id}/sink",
                    pathParams: new Dictionary<string, string> { { "stream_id", streamId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: streamSinkIn
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
        /// Get a sink by id or uid.
        /// </summary>
        public async Task<StreamSinkOut> GetAsync(
            string streamId,
            string sinkId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<StreamSinkOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
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
        /// Get a sink by id or uid.
        /// </summary>
        public StreamSinkOut Get(string streamId, string sinkId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<StreamSinkOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    }
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
        /// Update a sink.
        /// </summary>
        public async Task<StreamSinkOut> UpdateAsync(
            string streamId,
            string sinkId,
            StreamSinkIn streamSinkIn,
            CancellationToken cancellationToken = default
        )
        {
            streamSinkIn = streamSinkIn ?? throw new ArgumentNullException(nameof(streamSinkIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<StreamSinkOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
                    content: streamSinkIn,
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
        /// Update a sink.
        /// </summary>
        public StreamSinkOut Update(string streamId, string sinkId, StreamSinkIn streamSinkIn)
        {
            streamSinkIn = streamSinkIn ?? throw new ArgumentNullException(nameof(streamSinkIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<StreamSinkOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
                    content: streamSinkIn
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
        /// Delete a sink.
        /// </summary>
        public async Task<bool> DeleteAsync(
            string streamId,
            string sinkId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
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
        /// Delete a sink.
        /// </summary>
        public bool Delete(string streamId, string sinkId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    }
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
        /// Partially update a sink.
        /// </summary>
        public async Task<StreamSinkOut> PatchAsync(
            string streamId,
            string sinkId,
            StreamSinkPatch streamSinkPatch,
            CancellationToken cancellationToken = default
        )
        {
            streamSinkPatch =
                streamSinkPatch ?? throw new ArgumentNullException(nameof(streamSinkPatch));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<StreamSinkOut>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
                    content: streamSinkPatch,
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
        /// Partially update a sink.
        /// </summary>
        public StreamSinkOut Patch(string streamId, string sinkId, StreamSinkPatch streamSinkPatch)
        {
            streamSinkPatch =
                streamSinkPatch ?? throw new ArgumentNullException(nameof(streamSinkPatch));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<StreamSinkOut>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
                    content: streamSinkPatch
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Patch)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get the sink's signing secret (only supported for http sinks)
        ///
        /// This is used to verify the authenticity of the delivery.
        ///
        /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
        /// </summary>
        public async Task<SinkSecretOut> GetSecretAsync(
            string streamId,
            string sinkId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<SinkSecretOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}/secret",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(GetSecretAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get the sink's signing secret (only supported for http sinks)
        ///
        /// This is used to verify the authenticity of the delivery.
        ///
        /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
        /// </summary>
        public SinkSecretOut GetSecret(string streamId, string sinkId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<SinkSecretOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}/secret",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    }
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(GetSecret)} failed");

                throw;
            }
        }

        /// <summary>
        /// Rotates the signing secret (only supported for http sinks).
        /// </summary>
        public async Task<EmptyResponse> RotateSecretAsync(
            string streamId,
            string sinkId,
            EndpointSecretRotateIn endpointSecretRotateIn,
            StreamingSinkRotateSecretOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            endpointSecretRotateIn =
                endpointSecretRotateIn
                ?? throw new ArgumentNullException(nameof(endpointSecretRotateIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<EmptyResponse>(
                    method: HttpMethod.Post,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}/secret/rotate",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: endpointSecretRotateIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(RotateSecretAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Rotates the signing secret (only supported for http sinks).
        /// </summary>
        public EmptyResponse RotateSecret(
            string streamId,
            string sinkId,
            EndpointSecretRotateIn endpointSecretRotateIn,
            StreamingSinkRotateSecretOptions? options = null
        )
        {
            endpointSecretRotateIn =
                endpointSecretRotateIn
                ?? throw new ArgumentNullException(nameof(endpointSecretRotateIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<EmptyResponse>(
                    method: HttpMethod.Post,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}/secret/rotate",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: endpointSecretRotateIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(RotateSecret)} failed");

                throw;
            }
        }

        /// <summary>
        /// Set or unset the transformation code associated with this sink.
        /// </summary>
        public async Task<EmptyResponse> TransformationPartialUpdateAsync(
            string streamId,
            string sinkId,
            SinkTransformIn sinkTransformIn,
            CancellationToken cancellationToken = default
        )
        {
            sinkTransformIn =
                sinkTransformIn ?? throw new ArgumentNullException(nameof(sinkTransformIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<EmptyResponse>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
                    content: sinkTransformIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(TransformationPartialUpdateAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Set or unset the transformation code associated with this sink.
        /// </summary>
        public EmptyResponse TransformationPartialUpdate(
            string streamId,
            string sinkId,
            SinkTransformIn sinkTransformIn
        )
        {
            sinkTransformIn =
                sinkTransformIn ?? throw new ArgumentNullException(nameof(sinkTransformIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<EmptyResponse>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
                    content: sinkTransformIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(TransformationPartialUpdate)} failed");

                throw;
            }
        }
    }
}
