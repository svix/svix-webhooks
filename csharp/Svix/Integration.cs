// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class IntegrationListOptions : SvixOptionsBase
    {
        public ulong? Limit { get; set; }
        public string? Iterator { get; set; }
        public Ordering? Order { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?>
                {
                    { "limit", Limit },
                    { "iterator", Iterator },
                    { "order", Order },
                }
            );
        }
    }

    public class IntegrationCreateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class IntegrationRotateKeyOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class Integration(SvixClient client)
    {
        readonly SvixClient _client = client;

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
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<ListResponseIntegrationOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/app/{app_id}/integration",
                        pathParams: new Dictionary<string, string> { { "app_id", appId } },
                        queryParams: options?.QueryParams(),
                        headerParams: options?.HeaderParams(),
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// List the application's integrations.
        /// </summary>
        public ListResponseIntegrationOut List(string appId, IntegrationListOptions? options = null)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ListResponseIntegrationOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/integration",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(List)} failed");

                throw;
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
            integrationIn = integrationIn ?? throw new ArgumentNullException(nameof(integrationIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<IntegrationOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/integration",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: integrationIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                throw;
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
            integrationIn = integrationIn ?? throw new ArgumentNullException(nameof(integrationIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<IntegrationOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/integration",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: integrationIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Create)} failed");

                throw;
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
                var response = await _client.SvixHttpClient.SendRequestAsync<IntegrationOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/integration/{integ_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "integ_id", integId },
                    },
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
        /// Get an integration.
        /// </summary>
        public IntegrationOut Get(string appId, string integId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<IntegrationOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/integration/{integ_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "integ_id", integId },
                    }
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Get)} failed");

                throw;
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
            integrationUpdate =
                integrationUpdate ?? throw new ArgumentNullException(nameof(integrationUpdate));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<IntegrationOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/app/{app_id}/integration/{integ_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "integ_id", integId },
                    },
                    content: integrationUpdate,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(UpdateAsync)} failed");

                throw;
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
            integrationUpdate =
                integrationUpdate ?? throw new ArgumentNullException(nameof(integrationUpdate));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<IntegrationOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/app/{app_id}/integration/{integ_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "integ_id", integId },
                    },
                    content: integrationUpdate
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Update)} failed");

                throw;
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
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/app/{app_id}/integration/{integ_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "integ_id", integId },
                    },
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(DeleteAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Delete an integration.
        /// </summary>
        public bool Delete(string appId, string integId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/app/{app_id}/integration/{integ_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "integ_id", integId },
                    }
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Delete)} failed");

                throw;
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
                var response = await _client.SvixHttpClient.SendRequestAsync<IntegrationKeyOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/integration/{integ_id}/key",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "integ_id", integId },
                    },
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(GetKeyAsync)} failed");

                throw;
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
                var response = _client.SvixHttpClient.SendRequest<IntegrationKeyOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/integration/{integ_id}/key",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "integ_id", integId },
                    }
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(GetKey)} failed");

                throw;
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
                var response = await _client.SvixHttpClient.SendRequestAsync<IntegrationKeyOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/integration/{integ_id}/key/rotate",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "integ_id", integId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(RotateKeyAsync)} failed");

                throw;
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
                var response = _client.SvixHttpClient.SendRequest<IntegrationKeyOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/integration/{integ_id}/key/rotate",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "integ_id", integId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(RotateKey)} failed");

                throw;
            }
        }
    }
}
