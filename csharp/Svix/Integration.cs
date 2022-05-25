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
                var lIntegration = _integrationApi.CreateIntegrationApiV1AppAppIdIntegrationPost(
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
                var lIntegration = await _integrationApi.CreateIntegrationApiV1AppAppIdIntegrationPostAsync(
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
                var lResponse = _integrationApi.DeleteIntegrationApiV1AppAppIdIntegrationIntegIdDeleteWithHttpInfo(
                    integrationId,
                    appId,
                    idempotencyKey);

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
                var lResponse = await _integrationApi.DeleteIntegrationApiV1AppAppIdIntegrationIntegIdDeleteWithHttpInfoAsync(
                    integrationId,
                    appId,
                    idempotencyKey,
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
                var lIntegration = _integrationApi.GetIntegrationApiV1AppAppIdIntegrationIntegIdGet(
                    integrationId,
                    appId,
                    idempotencyKey);

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
                var lIntegration = await _integrationApi.GetIntegrationApiV1AppAppIdIntegrationIntegIdGetAsync(
                    integrationId,
                    appId,
                    idempotencyKey,
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
                var lResponse = _integrationApi.GetIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyGet(
                    integrationId,
                    appId,
                    idempotencyKey);

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
                var lResponse = await _integrationApi.GetIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyGetAsync(
                    integrationId,
                    appId,
                    idempotencyKey,
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

        public List<IntegrationOut> List(string appId, ListOptions options = null, string idempotencyKey = default)
        {
            try
            {
                var lResult = _integrationApi.ListIntegrationsApiV1AppAppIdIntegrationGet(
                    appId,
                    options?.Iterator,
                    options?.Limit,
                    idempotencyKey);

                return lResult?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;

                return new List<IntegrationOut>();
            }
        }

        public async Task<List<IntegrationOut>> ListAsync(string appId, ListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResult = await _integrationApi.ListIntegrationsApiV1AppAppIdIntegrationGetAsync(
                    appId,
                    options?.Iterator,
                    options?.Limit,
                    idempotencyKey,
                    cancellationToken);

                return lResult?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;

                return new List<IntegrationOut>();
            }
        }

        public string RotateKey(string appId, string integrationId, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _integrationApi.RotateIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyRotatePost(
                    integrationId,
                    appId,
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
                var lResponse = await _integrationApi.RotateIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyRotatePostAsync(
                    integrationId,
                    appId,
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
                var lIntegration = _integrationApi.UpdateIntegrationApiV1AppAppIdIntegrationIntegIdPut(
                    integrationId,
                    appId,
                    integration,
                    idempotencyKey);

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
                var lIntegration = await _integrationApi.UpdateIntegrationApiV1AppAppIdIntegrationIntegIdPutAsync(
                    integrationId,
                    appId,
                    integration,
                    idempotencyKey,
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
