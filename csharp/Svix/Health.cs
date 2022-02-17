using Microsoft.Extensions.Logging;
using Svix.Abstractions;
using Svix.Api;
using Svix.Client;
using System;
using System.Net;
using System.Threading;
using System.Threading.Tasks;

namespace Svix
{
    public sealed class Health : IHealth
    {
        private Configuration Config => new Configuration
        {
            BasePath = _svixClient.ServerUrl,
            AccessToken = _svixClient.Token
        };
        
        private readonly SvixClient _svixClient;
        
        public Health(SvixClient svixClient)
        {
            _svixClient = svixClient ?? throw new ArgumentNullException(nameof(svixClient));
        }

        public bool IsHealthy(string idempotencyKey = default)
        {
            try
            {
                using var lHealthApi = new HealthApi(Config);
                var lResponse = lHealthApi.HealthApiV1HealthGetWithHttpInfo(idempotencyKey);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                _svixClient.Logger
                    ?.LogError(e, $"{nameof(IsHealthy)} failed");

                if (_svixClient.Throw)
                    throw;
                
                return false;
            }
        }

        public async Task<bool> IsHealthyAsync(string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                using var lHealthApi = new HealthApi(Config);
                var lResponse = await lHealthApi.HealthApiV1HealthGetWithHttpInfoAsync(idempotencyKey, cancellationToken)
                    .ConfigureAwait(false);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                _svixClient.Logger
                    ?.LogError(e, $"{nameof(IsHealthyAsync)} failed");
                
                if (_svixClient.Throw)
                    throw;
                
                return false;
            }
        }
    }
}