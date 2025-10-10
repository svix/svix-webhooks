// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class Streaming(SvixClient client)
    {
        readonly SvixClient _client = client;

        public StreamingEventType EventType
        {
            get => new StreamingEventType(_client);
        }

        public StreamingEvents Events
        {
            get => new StreamingEvents(_client);
        }

        public StreamingSink Sink
        {
            get => new StreamingSink(_client);
        }

        public StreamingStream Stream
        {
            get => new StreamingStream(_client);
        }

        /// <summary>
        /// Get the HTTP sink headers. Only valid for `http` or `otelTracing` sinks.
        /// </summary>
        public async Task<EndpointHeadersOut> SinkHeadersGetAsync(
            string streamId,
            string sinkId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<EndpointHeadersOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}/headers",
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
                _client.Logger?.LogError(e, $"{nameof(SinkHeadersGetAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get the HTTP sink headers. Only valid for `http` or `otelTracing` sinks.
        /// </summary>
        public EndpointHeadersOut SinkHeadersGet(string streamId, string sinkId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<EndpointHeadersOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}/headers",
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
                _client.Logger?.LogError(e, $"{nameof(SinkHeadersGet)} failed");

                throw;
            }
        }

        /// <summary>
        /// Updates the Sink's headers. Only valid for `http` or `otelTracing` sinks.
        /// </summary>
        public async Task<EndpointHeadersOut> SinkHeadersPatchAsync(
            string streamId,
            string sinkId,
            HttpSinkHeadersPatchIn httpSinkHeadersPatchIn,
            CancellationToken cancellationToken = default
        )
        {
            httpSinkHeadersPatchIn =
                httpSinkHeadersPatchIn
                ?? throw new ArgumentNullException(nameof(httpSinkHeadersPatchIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<EndpointHeadersOut>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}/headers",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
                    content: httpSinkHeadersPatchIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(SinkHeadersPatchAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Updates the Sink's headers. Only valid for `http` or `otelTracing` sinks.
        /// </summary>
        public EndpointHeadersOut SinkHeadersPatch(
            string streamId,
            string sinkId,
            HttpSinkHeadersPatchIn httpSinkHeadersPatchIn
        )
        {
            httpSinkHeadersPatchIn =
                httpSinkHeadersPatchIn
                ?? throw new ArgumentNullException(nameof(httpSinkHeadersPatchIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<EndpointHeadersOut>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}/headers",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
                    content: httpSinkHeadersPatchIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(SinkHeadersPatch)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get the transformation code associated with this sink.
        /// </summary>
        public async Task<SinkTransformationOut> SinkTransformationGetAsync(
            string streamId,
            string sinkId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<SinkTransformationOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
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
                _client.Logger?.LogError(e, $"{nameof(SinkTransformationGetAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get the transformation code associated with this sink.
        /// </summary>
        public SinkTransformationOut SinkTransformationGet(string streamId, string sinkId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<SinkTransformationOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
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
                _client.Logger?.LogError(e, $"{nameof(SinkTransformationGet)} failed");

                throw;
            }
        }
    }
}
