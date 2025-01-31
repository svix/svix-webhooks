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
    public partial class IntegrationListOptions
    {
        public int? Limit { get; set; }
        public string? Iterator { get; set; }
        public Ordering? Order { get; set; }
    }

    public partial class IntegrationCreateOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public partial class IntegrationRotateKeyOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public sealed class Integration : SvixResourceBase
    {
        private readonly IntegrationApi _integrationApi;

        public Integration(ISvixClient svixClient, IntegrationApi integrationApi)
            : base(svixClient)
        {
            _integrationApi =
                integrationApi ?? throw new ArgumentNullException(nameof(integrationApi));
        }

        /// <summary>
        /// List the application's integrations.
        /// </summary>
        public async Task<ListResponseIntegrationOut> ListAsync(
            string appId,
            IntegrationListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _integrationApi.V1IntegrationListWithHttpInfoAsync(
                    appId: appId,
                    limit: options?.Limit,
                    iterator: options?.Iterator,
                    order: options?.Order,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;
                return new ListResponseIntegrationOut();
            }
        }

        /// <summary>
        /// List the application's integrations.
        /// </summary>
        public ListResponseIntegrationOut List(string appId, IntegrationListOptions? options = null)
        {
            try
            {
                var response = _integrationApi.V1IntegrationListWithHttpInfo(
                    appId: appId,
                    limit: options?.Limit,
                    iterator: options?.Iterator,
                    order: options?.Order
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;
                return new ListResponseIntegrationOut();
            }
        }

        /// <summary>
        /// Create an integration.
        /// </summary>
        public async Task<IntegrationOut> CreateAsync(
            string appId,
            IntegrationIn integrationIn,
            IntegrationCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                integrationIn =
                    integrationIn ?? throw new ArgumentNullException(nameof(integrationIn));
                var response = await _integrationApi.V1IntegrationCreateWithHttpInfoAsync(
                    appId: appId,
                    integrationIn: integrationIn,
                    idempotencyKey: options?.IdempotencyKey,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                if (Throw)
                    throw;
                return new IntegrationOut();
            }
        }

        /// <summary>
        /// Create an integration.
        /// </summary>
        public IntegrationOut Create(
            string appId,
            IntegrationIn integrationIn,
            IntegrationCreateOptions? options = null
        )
        {
            try
            {
                integrationIn =
                    integrationIn ?? throw new ArgumentNullException(nameof(integrationIn));
                var response = _integrationApi.V1IntegrationCreateWithHttpInfo(
                    appId: appId,
                    integrationIn: integrationIn,
                    idempotencyKey: options?.IdempotencyKey
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Create)} failed");

                if (Throw)
                    throw;
                return new IntegrationOut();
            }
        }

        /// <summary>
        /// Get an integration.
        /// </summary>
        public async Task<IntegrationOut> GetAsync(
            string appId,
            string integId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _integrationApi.V1IntegrationGetWithHttpInfoAsync(
                    appId: appId,
                    integId: integId,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                if (Throw)
                    throw;
                return new IntegrationOut();
            }
        }

        /// <summary>
        /// Get an integration.
        /// </summary>
        public IntegrationOut Get(string appId, string integId)
        {
            try
            {
                var response = _integrationApi.V1IntegrationGetWithHttpInfo(
                    appId: appId,
                    integId: integId
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Get)} failed");

                if (Throw)
                    throw;
                return new IntegrationOut();
            }
        }

        /// <summary>
        /// Update an integration.
        /// </summary>
        public async Task<IntegrationOut> UpdateAsync(
            string appId,
            string integId,
            IntegrationUpdate integrationUpdate,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                integrationUpdate =
                    integrationUpdate ?? throw new ArgumentNullException(nameof(integrationUpdate));
                var response = await _integrationApi.V1IntegrationUpdateWithHttpInfoAsync(
                    appId: appId,
                    integId: integId,
                    integrationUpdate: integrationUpdate,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(UpdateAsync)} failed");

                if (Throw)
                    throw;
                return new IntegrationOut();
            }
        }

        /// <summary>
        /// Update an integration.
        /// </summary>
        public IntegrationOut Update(
            string appId,
            string integId,
            IntegrationUpdate integrationUpdate
        )
        {
            try
            {
                integrationUpdate =
                    integrationUpdate ?? throw new ArgumentNullException(nameof(integrationUpdate));
                var response = _integrationApi.V1IntegrationUpdateWithHttpInfo(
                    appId: appId,
                    integId: integId,
                    integrationUpdate: integrationUpdate
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Update)} failed");

                if (Throw)
                    throw;
                return new IntegrationOut();
            }
        }

        /// <summary>
        /// Delete an integration.
        /// </summary>
        public async Task<bool> DeleteAsync(
            string appId,
            string integId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _integrationApi.V1IntegrationDeleteWithHttpInfoAsync(
                    appId: appId,
                    integId: integId,
                    cancellationToken: cancellationToken
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(DeleteAsync)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }

        /// <summary>
        /// Delete an integration.
        /// </summary>
        public bool Delete(string appId, string integId)
        {
            try
            {
                var response = _integrationApi.V1IntegrationDeleteWithHttpInfo(
                    appId: appId,
                    integId: integId
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Delete)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }

        /// <summary>
        /// Get an integration's key.
        /// </summary>
        [Obsolete]
        public async Task<IntegrationKeyOut> GetKeyAsync(
            string appId,
            string integId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _integrationApi.V1IntegrationGetKeyWithHttpInfoAsync(
                    appId: appId,
                    integId: integId,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetKeyAsync)} failed");

                if (Throw)
                    throw;
                return new IntegrationKeyOut();
            }
        }

        /// <summary>
        /// Get an integration's key.
        /// </summary>
        [Obsolete]
        public IntegrationKeyOut GetKey(string appId, string integId)
        {
            try
            {
                var response = _integrationApi.V1IntegrationGetKeyWithHttpInfo(
                    appId: appId,
                    integId: integId
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetKey)} failed");

                if (Throw)
                    throw;
                return new IntegrationKeyOut();
            }
        }

        /// <summary>
        /// Rotate the integration's key. The previous key will be immediately revoked.
        /// </summary>
        public async Task<IntegrationKeyOut> RotateKeyAsync(
            string appId,
            string integId,
            IntegrationRotateKeyOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _integrationApi.V1IntegrationRotateKeyWithHttpInfoAsync(
                    appId: appId,
                    integId: integId,
                    idempotencyKey: options?.IdempotencyKey,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(RotateKeyAsync)} failed");

                if (Throw)
                    throw;
                return new IntegrationKeyOut();
            }
        }

        /// <summary>
        /// Rotate the integration's key. The previous key will be immediately revoked.
        /// </summary>
        public IntegrationKeyOut RotateKey(
            string appId,
            string integId,
            IntegrationRotateKeyOptions? options = null
        )
        {
            try
            {
                var response = _integrationApi.V1IntegrationRotateKeyWithHttpInfo(
                    appId: appId,
                    integId: integId,
                    idempotencyKey: options?.IdempotencyKey
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(RotateKey)} failed");

                if (Throw)
                    throw;
                return new IntegrationKeyOut();
            }
        }
    }
}
