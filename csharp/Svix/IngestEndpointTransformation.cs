// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class IngestEndpointTransformation(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Get the transformation code associated with this ingest endpoint.
        /// </summary>
        public async Task<IngestEndpointTransformationOut> TransformationAsync(
            string sourceId,
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<IngestEndpointTransformationOut>(
                        method: HttpMethod.Get,
                        path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation",
                        pathParams: new Dictionary<string, string>
                        {
                            { "source_id", sourceId },
                            { "endpoint_id", endpointId },
                        },
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(TransformationAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get the transformation code associated with this ingest endpoint.
        /// </summary>
        public IngestEndpointTransformationOut Transformation(string sourceId, string endpointId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<IngestEndpointTransformationOut>(
                    method: HttpMethod.Get,
                    path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation",
                    pathParams: new Dictionary<string, string>
                    {
                        { "source_id", sourceId },
                        { "endpoint_id", endpointId },
                    }
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Transformation)} failed");

                throw;
            }
        }

        /// <summary>
        /// Set or unset the transformation code associated with this ingest endpoint.
        /// </summary>
        public async Task<bool> PatchAsync(
            string sourceId,
            string endpointId,
            IngestEndpointTransformationPatch ingestEndpointTransformationPatch,
            CancellationToken cancellationToken = default
        )
        {
            ingestEndpointTransformationPatch =
                ingestEndpointTransformationPatch
                ?? throw new ArgumentNullException(nameof(ingestEndpointTransformationPatch));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Patch,
                    path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation",
                    pathParams: new Dictionary<string, string>
                    {
                        { "source_id", sourceId },
                        { "endpoint_id", endpointId },
                    },
                    content: ingestEndpointTransformationPatch,
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
        /// Set or unset the transformation code associated with this ingest endpoint.
        /// </summary>
        public bool Patch(
            string sourceId,
            string endpointId,
            IngestEndpointTransformationPatch ingestEndpointTransformationPatch
        )
        {
            ingestEndpointTransformationPatch =
                ingestEndpointTransformationPatch
                ?? throw new ArgumentNullException(nameof(ingestEndpointTransformationPatch));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Patch,
                    path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation",
                    pathParams: new Dictionary<string, string>
                    {
                        { "source_id", sourceId },
                        { "endpoint_id", endpointId },
                    },
                    content: ingestEndpointTransformationPatch
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
