// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class StreamingSinkTransformation(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Get the transformation code associated with this sink.
        /// </summary>
        public async Task<SinkTransformationOut> GetAsync(
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
                _client.Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get the transformation code associated with this sink.
        /// </summary>
        public SinkTransformationOut Get(string streamId, string sinkId)
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
                _client.Logger?.LogError(e, $"{nameof(Get)} failed");

                throw;
            }
        }

        /// <summary>
        /// Set or unset the transformation code associated with this sink.
        /// </summary>
        public async Task<EmptyResponse> PatchAsync(
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
                _client.Logger?.LogError(e, $"{nameof(PatchAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Set or unset the transformation code associated with this sink.
        /// </summary>
        public EmptyResponse Patch(string streamId, string sinkId, SinkTransformIn sinkTransformIn)
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
                _client.Logger?.LogError(e, $"{nameof(Patch)} failed");

                throw;
            }
        }
    }
}
