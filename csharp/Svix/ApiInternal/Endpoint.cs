// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix.ApiInternal
{
    public class Endpoint(SvixClient client)
    {
        readonly SvixClient _client = client;

        public EndpointAutoConfig AutoConfig
        {
            get => new EndpointAutoConfig(_client);
        }

        /// <summary>
        /// This operation was renamed to `set-transformation`.
        /// </summary>
        [Obsolete]
        public async Task<bool> TransformationPartialUpdateAsync(
            string appId,
            string endpointId,
            EndpointTransformationIn endpointTransformationIn,
            CancellationToken cancellationToken = default
        )
        {
            endpointTransformationIn =
                endpointTransformationIn
                ?? throw new ArgumentNullException(nameof(endpointTransformationIn));
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
                    content: endpointTransformationIn,
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
        /// This operation was renamed to `set-transformation`.
        /// </summary>
        [Obsolete]
        public bool TransformationPartialUpdate(
            string appId,
            string endpointId,
            EndpointTransformationIn endpointTransformationIn
        )
        {
            endpointTransformationIn =
                endpointTransformationIn
                ?? throw new ArgumentNullException(nameof(endpointTransformationIn));
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
                    content: endpointTransformationIn
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
