// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class EndpointTransformation(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Get the transformation code associated with this endpoint.
        /// </summary>
        public async Task<EndpointTransformationOut> GetAsync(
            string appId,
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<EndpointTransformationOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
                        pathParams: new Dictionary<string, string>
                        {
                            { "app_id", appId },
                            { "endpoint_id", endpointId },
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
        /// Get the transformation code associated with this endpoint.
        /// </summary>
        public EndpointTransformationOut Get(string appId, string endpointId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<EndpointTransformationOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "endpoint_id", endpointId },
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
        /// Set or unset the transformation code associated with this endpoint.
        /// </summary>
        public async Task<bool> PatchAsync(
            string appId,
            string endpointId,
            EndpointTransformationPatch endpointTransformationPatch,
            CancellationToken cancellationToken = default
        )
        {
            endpointTransformationPatch =
                endpointTransformationPatch
                ?? throw new ArgumentNullException(nameof(endpointTransformationPatch));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "endpoint_id", endpointId },
                    },
                    content: endpointTransformationPatch,
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
        /// Set or unset the transformation code associated with this endpoint.
        /// </summary>
        public bool Patch(
            string appId,
            string endpointId,
            EndpointTransformationPatch endpointTransformationPatch
        )
        {
            endpointTransformationPatch =
                endpointTransformationPatch
                ?? throw new ArgumentNullException(nameof(endpointTransformationPatch));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "endpoint_id", endpointId },
                    },
                    content: endpointTransformationPatch
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
