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
    public sealed class OperationalWebhookEndpoint : SvixResourceBase, IOperationalWebhookEndpoint
    {
        private readonly IWebhookEndpointApi _opWebhookEndpointApi;

        public OperationalWebhookEndpoint(ISvixClient svixClient, IWebhookEndpointApi endpointApi)
            : base(svixClient)
        {
            _opWebhookEndpointApi = endpointApi ?? throw new ArgumentNullException(nameof(_opWebhookEndpointApi));
        }

        public OperationalWebhookEndpointOut Create(
            OperationalWebhookEndpointIn endpoint, string idempotencyKey = default)
        {
            try
            {
                var lEndpoint = _opWebhookEndpointApi.CreateOperationalWebhookEndpoint(
                    endpoint,
                    idempotencyKey);

                return lEndpoint;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Create)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<OperationalWebhookEndpointOut> CreateAsync(
            OperationalWebhookEndpointIn endpoint, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lEndpoint = await _opWebhookEndpointApi.CreateOperationalWebhookEndpointAsync(
                    endpoint,
                    idempotencyKey,
                    cancellationToken);

                return lEndpoint;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public bool Delete(string endpointId, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _opWebhookEndpointApi.DeleteOperationalWebhookEndpointWithHttpInfo(
                    endpointId);

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

        public async Task<bool> DeleteAsync(string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _opWebhookEndpointApi.DeleteOperationalWebhookEndpointWithHttpInfoAsync(
                    endpointId,
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

        public OperationalWebhookEndpointOut Get(string endpointId, string idempotencyKey = default)
        {
            try
            {
                var lEndpoint = _opWebhookEndpointApi.GetOperationalWebhookEndpoint(endpointId);
                return lEndpoint;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Get)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<OperationalWebhookEndpointOut> GetAsync(string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lEndpoint = await _opWebhookEndpointApi.GetOperationalWebhookEndpointAsync(
                    endpointId,
                    cancellationToken);

                return lEndpoint;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public string GetSecret(string endpointId, string idempotencyKey = default)
        {
            try
            {
                var lSecret = _opWebhookEndpointApi.GetOperationalWebhookEndpointSecret(
                    endpointId);

                return lSecret?.Key;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetSecret)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<string> GetSecretAsync(string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lSecret = await _opWebhookEndpointApi.GetOperationalWebhookEndpointSecretAsync(
                    endpointId,
                    cancellationToken);

                return lSecret.Key;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetSecretAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public ListResponseOperationalWebhookEndpointOut List(ListOptions options = null,
            string idempotencyKey = default)
        {
            try
            {
                var lEndpoints = _opWebhookEndpointApi.ListOperationalWebhookEndpoints(
                    options?.Limit,
                    options?.Iterator,
                    options?.Order);

                return lEndpoints;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;

                return new ListResponseOperationalWebhookEndpointOut();
            }
        }

        public async Task<ListResponseOperationalWebhookEndpointOut> ListAsync(
            ListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lEndpoints = await _opWebhookEndpointApi.ListOperationalWebhookEndpointsAsync(
                    options?.Limit,
                    options?.Iterator,
                    options?.Order,
                    cancellationToken);

                return lEndpoints;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;

                return new ListResponseOperationalWebhookEndpointOut();
            }
        }

        public bool RotateSecret(string endpointId, OperationalWebhookEndpointSecretIn secret, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _opWebhookEndpointApi.RotateOperationalWebhookEndpointSecretWithHttpInfo(
                    endpointId,
                    secret,
                    idempotencyKey);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(RotateSecret)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public async Task<bool> RotateSecretAsync(string endpointId, OperationalWebhookEndpointSecretIn secret,
            string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _opWebhookEndpointApi.RotateOperationalWebhookEndpointSecretWithHttpInfoAsync(
                    endpointId,
                    secret,
                    idempotencyKey);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(RotateSecretAsync)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public OperationalWebhookEndpointOut Update(string endpointId,
            OperationalWebhookEndpointUpdate endpoint, string idempotencyKey = default)
        {
            try
            {
                var lEndpoint = _opWebhookEndpointApi.UpdateOperationalWebhookEndpoint(
                    endpointId,
                    endpoint);

                return lEndpoint;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Update)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<OperationalWebhookEndpointOut> UpdateAsync(string endpointId,
            OperationalWebhookEndpointUpdate endpoint, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lEndpoint = await _opWebhookEndpointApi.UpdateOperationalWebhookEndpointAsync(
                    endpointId,
                    endpoint,
                    cancellationToken);

                return lEndpoint;
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
