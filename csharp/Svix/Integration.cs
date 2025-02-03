// this file is @generated
#nullable enable
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
            var response =
                await this._client.SvixHttpClient.SendRequestAsync<ListResponseIntegrationOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/integration",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
            return response.Data;
        }

        /// <summary>
        /// List the application's integrations.
        /// </summary>
        public ListResponseIntegrationOut List(string appId, IntegrationListOptions? options = null)
        {
            var response = this._client.SvixHttpClient.SendRequest<ListResponseIntegrationOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/integration",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams()
            );
            return response.Data;
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

            var response = await this._client.SvixHttpClient.SendRequestAsync<IntegrationOut>(
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

            var response = this._client.SvixHttpClient.SendRequest<IntegrationOut>(
                method: HttpMethod.Post,
                path: "/api/v1/app/{app_id}/integration",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: integrationIn
            );
            return response.Data;
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
            var response = await this._client.SvixHttpClient.SendRequestAsync<IntegrationOut>(
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

        /// <summary>
        /// Get an integration.
        /// </summary>
        public IntegrationOut Get(string appId, string integId)
        {
            var response = this._client.SvixHttpClient.SendRequest<IntegrationOut>(
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

            var response = await this._client.SvixHttpClient.SendRequestAsync<IntegrationOut>(
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

            var response = this._client.SvixHttpClient.SendRequest<IntegrationOut>(
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

        /// <summary>
        /// Delete an integration.
        /// </summary>
        public async Task<bool> DeleteAsync(
            string appId,
            string integId,
            CancellationToken cancellationToken = default
        )
        {
            var response = await this._client.SvixHttpClient.SendRequestAsync<bool>(
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

        /// <summary>
        /// Delete an integration.
        /// </summary>
        public bool Delete(string appId, string integId)
        {
            var response = this._client.SvixHttpClient.SendRequest<bool>(
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
            var response = await this._client.SvixHttpClient.SendRequestAsync<IntegrationKeyOut>(
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

        /// <summary>
        /// Get an integration's key.
        /// </summary>
        [Obsolete]
        public IntegrationKeyOut GetKey(string appId, string integId)
        {
            var response = this._client.SvixHttpClient.SendRequest<IntegrationKeyOut>(
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
            var response = await this._client.SvixHttpClient.SendRequestAsync<IntegrationKeyOut>(
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

        /// <summary>
        /// Rotate the integration's key. The previous key will be immediately revoked.
        /// </summary>
        public IntegrationKeyOut RotateKey(
            string appId,
            string integId,
            IntegrationRotateKeyOptions? options = null
        )
        {
            var response = this._client.SvixHttpClient.SendRequest<IntegrationKeyOut>(
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
    }
}
