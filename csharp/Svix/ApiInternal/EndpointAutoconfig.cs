// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix.ApiInternal
{
    public class EndpointAutoconfig(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Create or update the HTTP endpoint for an AutoConfig subscription.
        /// </summary>
        public async Task<EndpointOut> SubscribeAsync(
            string appId,
            string autoconfigId,
            EndpointIn endpointIn,
            CancellationToken cancellationToken = default
        )
        {
            endpointIn = endpointIn ?? throw new ArgumentNullException(nameof(endpointIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<EndpointOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/endpoint",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "autoconfig_id", autoconfigId },
                    },
                    content: endpointIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(SubscribeAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Create or update the HTTP endpoint for an AutoConfig subscription.
        /// </summary>
        public EndpointOut Subscribe(string appId, string autoconfigId, EndpointIn endpointIn)
        {
            endpointIn = endpointIn ?? throw new ArgumentNullException(nameof(endpointIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<EndpointOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/endpoint",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "autoconfig_id", autoconfigId },
                    },
                    content: endpointIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Subscribe)} failed");

                throw;
            }
        }
    }
}
