// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix.ApiInternal
{
    public class DestinationAutoconfig(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Create or update the destination for an AutoConfig subscription.
        /// </summary>
        public async Task<DestinationOut> SubscribeAsync(
            string appId,
            string autoconfigId,
            DestinationIn destinationIn,
            CancellationToken cancellationToken = default
        )
        {
            destinationIn = destinationIn ?? throw new ArgumentNullException(nameof(destinationIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<DestinationOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/destination",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "autoconfig_id", autoconfigId },
                    },
                    content: destinationIn,
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
        /// Create or update the destination for an AutoConfig subscription.
        /// </summary>
        public DestinationOut Subscribe(
            string appId,
            string autoconfigId,
            DestinationIn destinationIn
        )
        {
            destinationIn = destinationIn ?? throw new ArgumentNullException(nameof(destinationIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<DestinationOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/destination",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "autoconfig_id", autoconfigId },
                    },
                    content: destinationIn
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
