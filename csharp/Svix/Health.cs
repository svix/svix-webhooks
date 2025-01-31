// this file is @generated
#nullable enable
using System;
using System.Collections.Generic;
using System.Net;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;
using Svix.Abstractions;
using Svix.Api;
using Svix.Client;
using Svix.Model;
using Svix.Models;

namespace Svix
{
    public sealed class Health : SvixResourceBase
    {
        private readonly HealthApi _healthApi;

        public Health(ISvixClient svixClient, HealthApi healthApi)
            : base(svixClient)
        {
            _healthApi = healthApi ?? throw new ArgumentNullException(nameof(healthApi));
        }

        /// <summary>
        /// Verify the API server is up and running.
        /// </summary>
        public async Task<bool> GetAsync(CancellationToken cancellationToken = default)
        {
            try
            {
                var response = await _healthApi.V1HealthGetWithHttpInfoAsync(
                    cancellationToken: cancellationToken
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }

        /// <summary>
        /// Verify the API server is up and running.
        /// </summary>
        public bool Get()
        {
            try
            {
                var response = _healthApi.V1HealthGetWithHttpInfo();
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Get)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }
    }
}
