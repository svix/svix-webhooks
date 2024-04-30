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
    public sealed class Integration : SvixResourceBase, IIntegration
    {
        private readonly IIntegrationApi _integrationApi;

        public Integration(ISvixClient svixClient, IIntegrationApi integrationApi) : base(svixClient)
        {
            _integrationApi = integrationApi ?? throw new ArgumentNullException(nameof(integrationApi));
        }

        public IntegrationOut Create(string appId, IntegrationIn integration, string idempotencyKey = default)
        {
            try
            {
                var lIntegration = _integrationApi.V1IntegrationCreate(
                    appId,
                    integration,
                    idempotencyKey);

                return lIntegration;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Create)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<IntegrationOut> CreateAsync(string appId, IntegrationIn integration, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lIntegration = await _integrationApi.V1IntegrationCreateAsync(
                    appId,
                    integration,
                    idempotencyKey,
                    cancellationToken);

                return lIntegration;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public bool Delete(string appId, string integrationId, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _integrationApi.V1IntegrationDeleteWithHttpInfo(
                    appId,
                    integrationId);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Delete)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public async Task<bool> DeleteAsync(string appId, string integrationId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _integrationApi.V1IntegrationDeleteWithHttpInfoAsync(
                    appId,
                    integrationId,
                    cancellationToken);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(DeleteAsync)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public IntegrationOut Get(string appId, string integrationId, string idempotencyKey = default)
        {
            try
            {
                var lIntegration = _integrationApi.V1IntegrationGet(
                    appId,
                    integrationId);

                return lIntegration;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Get)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<IntegrationOut> GetAsync(string appId, string integrationId, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lIntegration = await _integrationApi.V1IntegrationGetAsync(
                    appId,
                    integrationId,
                    cancellationToken);

                return lIntegration;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public string GetKey(string appId, string integrationId, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _integrationApi.V1IntegrationGetKey(
                    appId,
                    integrationId);

                return lResponse.Key;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetKey)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<string> GetKeyAsync(string appId, string integrationId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _integrationApi.V1IntegrationGetKeyAsync(
                    appId,
                    integrationId,
                    cancellationToken);

                return lResponse.Key;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetKeyAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public ListResponseIntegrationOut List(string appId, ListOptions options = null, string idempotencyKey = default)
        {
            try
            {
                var lResult = _integrationApi.V1IntegrationList(
                    appId,
                    options?.Limit,
                    options?.Iterator,
                    options?.Order);

                return lResult;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;

                return new ListResponseIntegrationOut();
            }
        }

        public async Task<ListResponseIntegrationOut> ListAsync(string appId, ListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResult = await _integrationApi.V1IntegrationListAsync(
                    appId,
                    options?.Limit,
                    options?.Iterator,
                    options?.Order,
                    cancellationToken);

                return lResult;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;

                return new ListResponseIntegrationOut();
            }
        }

        public string RotateKey(string appId, string integrationId, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _integrationApi.V1IntegrationRotateKey(
                    appId,
                    integrationId,
                    idempotencyKey);

                return lResponse.Key;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(RotateKey)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<string> RotateKeyAsync(string appId, string integrationId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _integrationApi.V1IntegrationRotateKeyAsync(
                    appId,
                    integrationId,
                    idempotencyKey,
                    cancellationToken);

                return lResponse.Key;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(RotateKeyAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public IntegrationOut Update(string appId, string integrationId, IntegrationUpdate integration, string idempotencyKey = default)
        {
            try
            {
                var lIntegration = _integrationApi.V1IntegrationUpdate(
                    appId,
                    integrationId,
                    integration);

                return lIntegration;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Update)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<IntegrationOut> UpdateAsync(string appId, string integrationId, IntegrationUpdate integration, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lIntegration = await _integrationApi.V1IntegrationUpdateAsync(
                    appId,
                    integrationId,
                    integration,
                    cancellationToken);

                return lIntegration;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(UpdateAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }
    }
}
