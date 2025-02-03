// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class Health(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Verify the API server is up and running.
        /// </summary>
        public async Task<bool> GetAsync(CancellationToken cancellationToken = default)
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Get,
                    path: "/api/v1/health",
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
        /// Verify the API server is up and running.
        /// </summary>
        public bool Get()
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Get,
                    path: "/api/v1/health"
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
