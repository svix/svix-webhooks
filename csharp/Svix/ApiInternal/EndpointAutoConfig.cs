// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix.ApiInternal
{
    public class EndpointAutoConfig(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Update an auto-config endpoint by providing endpoint details.
        /// </summary>
        public async Task<EndpointOut> UpdateAsync(
            string appId,
            string endpointId,
            SubscribeIn subscribeIn,
            CancellationToken cancellationToken = default
        )
        {
            subscribeIn = subscribeIn ?? throw new ArgumentNullException(nameof(subscribeIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<EndpointOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/auto-config",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "endpoint_id", endpointId },
                    },
                    content: subscribeIn,
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
        /// Update an auto-config endpoint by providing endpoint details.
        /// </summary>
        public EndpointOut Update(string appId, string endpointId, SubscribeIn subscribeIn)
        {
            subscribeIn = subscribeIn ?? throw new ArgumentNullException(nameof(subscribeIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<EndpointOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/auto-config",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "endpoint_id", endpointId },
                    },
                    content: subscribeIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Update)} failed");

                throw;
            }
        }
    }
}
