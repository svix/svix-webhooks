// this file is @generated
#nullable enable
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
            var response = await this._client.SvixHttpClient.SendRequestAsync<bool>(
                method: HttpMethod.Get,
                path: "/api/v1/health",
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Verify the API server is up and running.
        /// </summary>
        public bool Get()
        {
            var response = this._client.SvixHttpClient.SendRequest<bool>(
                method: HttpMethod.Get,
                path: "/api/v1/health"
            );
            return response.Data;
        }
    }
}
