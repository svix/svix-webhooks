using System;
using Microsoft.Extensions.Logging;
using Svix.Abstractions;
using Svix.Api;
using Svix.Client;
using System.Net;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Options;
using Svix.Models;

namespace Svix
{
    public sealed class Health : IHealth
    {
        private readonly ILogger _logger;

        private readonly SvixClientOptions _options;
        
        public Health(SvixClientOptions options, ILogger logger)
        {
            _logger = logger;
            _options = options ?? throw new ArgumentNullException(nameof(options));
        }
        
        public Health(IOptions<SvixClientOptions> options, ILogger<Health> logger)
            : this(options?.Value, logger)
        {
            // empty
        }
        
        public bool IsHealthy(string idempotencyKey = default)
        {
            try
            {
                var lConfig = new Configuration
                {
                    BasePath = _options.ServerUrl,
                    AccessToken = _options.AccessToken
                };
                
                using var lHealthApi = new HealthApi(lConfig);
                var lResponse = lHealthApi.HealthApiV1HealthGetWithHttpInfo(idempotencyKey);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                _logger?.LogError(e, $"{nameof(IsHealthy)} failed");

                if (_options.Throw)
                    throw;
                
                return false;
            }
        }

        public async Task<bool> IsHealthyAsync(string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lConfig = new Configuration
                {
                    BasePath = _options.ServerUrl,
                    AccessToken = _options.AccessToken
                };
                
                using var lHealthApi = new HealthApi(lConfig);
                var lResponse = await lHealthApi.HealthApiV1HealthGetWithHttpInfoAsync(idempotencyKey, cancellationToken)
                    .ConfigureAwait(false);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                _logger?.LogError(e, $"{nameof(IsHealthyAsync)} failed");
                
                if (_options.Throw)
                    throw;
                
                return false;
            }
        }
    }
}