// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix.Api
{
    public class DestinationTransformation(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Get the transformation code associated with this destination.
        /// </summary>
        public async Task<DestinationTransformationOut> GetAsync(
            string appId,
            string destinationId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<DestinationTransformationOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/app/{app_id}/destination/{destination_id}/transformation",
                        pathParams: new Dictionary<string, string>
                        {
                            { "app_id", appId },
                            { "destination_id", destinationId },
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
        /// Get the transformation code associated with this destination.
        /// </summary>
        public DestinationTransformationOut Get(string appId, string destinationId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<DestinationTransformationOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/destination/{destination_id}/transformation",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "destination_id", destinationId },
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
        /// Set or unset the transformation code associated with this destination.
        /// </summary>
        public async Task<EmptyResponse> PatchAsync(
            string appId,
            string destinationId,
            DestinationTransformIn destinationTransformIn,
            CancellationToken cancellationToken = default
        )
        {
            destinationTransformIn =
                destinationTransformIn
                ?? throw new ArgumentNullException(nameof(destinationTransformIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<EmptyResponse>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/app/{app_id}/destination/{destination_id}/transformation",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "destination_id", destinationId },
                    },
                    content: destinationTransformIn,
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
        /// Set or unset the transformation code associated with this destination.
        /// </summary>
        public EmptyResponse Patch(
            string appId,
            string destinationId,
            DestinationTransformIn destinationTransformIn
        )
        {
            destinationTransformIn =
                destinationTransformIn
                ?? throw new ArgumentNullException(nameof(destinationTransformIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<EmptyResponse>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/app/{app_id}/destination/{destination_id}/transformation",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "destination_id", destinationId },
                    },
                    content: destinationTransformIn
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
