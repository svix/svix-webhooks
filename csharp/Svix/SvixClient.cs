using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;
using Svix.Abstractions;
using Svix.Api;
using Svix.Client;
using Svix.Models;
using System;
using System.Net;
using System.Threading;
using System.Threading.Tasks;

namespace Svix
{
    public sealed class SvixClient : ISvixClient
    {
        private readonly ILogger _logger;
        
        private readonly SvixClientOptions _options;

        public SvixClient(SvixClientOptions options, ILogger<SvixClient> logger = null)
        {
            _options = options ?? throw new ArgumentNullException(nameof(options));
            _logger = logger;
        }

        public SvixClient(IOptions<SvixClientOptions> options, ILogger<SvixClient> logger)
            : this(options?.Value, logger)
        {
            // empty
        }
        
        #region " Health "
        
        public bool IsHealthy(string idempotencyKey = default)
        {
            try
            {
                var lConfig = new Configuration
                {
                    BasePath = _options.ServiceUrl,
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
                    BasePath = _options.ServiceUrl,
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
        
        #endregion
    }
}