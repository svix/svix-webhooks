// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix.ApiInternal
{
    public class AutoconfigSubscription(SvixClient client)
    {
        readonly SvixClient _client = client;

        public AutoconfigSubscriptionDestination Destination
        {
            get => new AutoconfigSubscriptionDestination(_client);
        }

        public AutoconfigSubscriptionEndpoint Endpoint
        {
            get => new AutoconfigSubscriptionEndpoint(_client);
        }

        /// <summary>
        /// Get an AutoConfig subscription, including the bound endpoint or destination if any.
        /// </summary>
        public async Task<AutoConfigSubscriptionOut> GetAsync(
            string appId,
            string autoconfigId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<AutoConfigSubscriptionOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}",
                        pathParams: new Dictionary<string, string>
                        {
                            { "app_id", appId },
                            { "autoconfig_id", autoconfigId },
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
        /// Get an AutoConfig subscription, including the bound endpoint or destination if any.
        /// </summary>
        public AutoConfigSubscriptionOut Get(string appId, string autoconfigId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<AutoConfigSubscriptionOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "autoconfig_id", autoconfigId },
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
    }
}
