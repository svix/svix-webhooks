// this file is @generated
#nullable enable
using Svix.Models;

namespace Svix
{
    public class OperationalWebhookEndpointListOptions : SvixOptionsBase
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

    public class OperationalWebhookEndpointCreateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class OperationalWebhookEndpointRotateSecretOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class OperationalWebhookEndpoint(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// List operational webhook endpoints.
        /// </summary>
        public async Task<ListResponseOperationalWebhookEndpointOut> ListAsync(
            OperationalWebhookEndpointListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            var response =
                await this._client.SvixHttpClient.SendRequestAsync<ListResponseOperationalWebhookEndpointOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/operational-webhook/endpoint",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
            return response.Data;
        }

        /// <summary>
        /// List operational webhook endpoints.
        /// </summary>
        public ListResponseOperationalWebhookEndpointOut List(
            OperationalWebhookEndpointListOptions? options = null
        )
        {
            var response =
                this._client.SvixHttpClient.SendRequest<ListResponseOperationalWebhookEndpointOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/operational-webhook/endpoint",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
            return response.Data;
        }

        /// <summary>
        /// Create an operational webhook endpoint.
        /// </summary>
        public async Task<OperationalWebhookEndpointOut> CreateAsync(
            OperationalWebhookEndpointIn operationalWebhookEndpointIn,
            OperationalWebhookEndpointCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            operationalWebhookEndpointIn =
                operationalWebhookEndpointIn
                ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointIn));

            var response =
                await this._client.SvixHttpClient.SendRequestAsync<OperationalWebhookEndpointOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/operational-webhook/endpoint",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: operationalWebhookEndpointIn,
                    cancellationToken: cancellationToken
                );
            return response.Data;
        }

        /// <summary>
        /// Create an operational webhook endpoint.
        /// </summary>
        public OperationalWebhookEndpointOut Create(
            OperationalWebhookEndpointIn operationalWebhookEndpointIn,
            OperationalWebhookEndpointCreateOptions? options = null
        )
        {
            operationalWebhookEndpointIn =
                operationalWebhookEndpointIn
                ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointIn));

            var response = this._client.SvixHttpClient.SendRequest<OperationalWebhookEndpointOut>(
                method: HttpMethod.Post,
                path: "/api/v1/operational-webhook/endpoint",
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: operationalWebhookEndpointIn
            );
            return response.Data;
        }

        /// <summary>
        /// Get an operational webhook endpoint.
        /// </summary>
        public async Task<OperationalWebhookEndpointOut> GetAsync(
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            var response =
                await this._client.SvixHttpClient.SendRequestAsync<OperationalWebhookEndpointOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/operational-webhook/endpoint/{endpoint_id}",
                    pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } },
                    cancellationToken: cancellationToken
                );
            return response.Data;
        }

        /// <summary>
        /// Get an operational webhook endpoint.
        /// </summary>
        public OperationalWebhookEndpointOut Get(string endpointId)
        {
            var response = this._client.SvixHttpClient.SendRequest<OperationalWebhookEndpointOut>(
                method: HttpMethod.Get,
                path: "/api/v1/operational-webhook/endpoint/{endpoint_id}",
                pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } }
            );
            return response.Data;
        }

        /// <summary>
        /// Update an operational webhook endpoint.
        /// </summary>
        public async Task<OperationalWebhookEndpointOut> UpdateAsync(
            string endpointId,
            OperationalWebhookEndpointUpdate operationalWebhookEndpointUpdate,
            CancellationToken cancellationToken = default
        )
        {
            operationalWebhookEndpointUpdate =
                operationalWebhookEndpointUpdate
                ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointUpdate));

            var response =
                await this._client.SvixHttpClient.SendRequestAsync<OperationalWebhookEndpointOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/operational-webhook/endpoint/{endpoint_id}",
                    pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } },
                    content: operationalWebhookEndpointUpdate,
                    cancellationToken: cancellationToken
                );
            return response.Data;
        }

        /// <summary>
        /// Update an operational webhook endpoint.
        /// </summary>
        public OperationalWebhookEndpointOut Update(
            string endpointId,
            OperationalWebhookEndpointUpdate operationalWebhookEndpointUpdate
        )
        {
            operationalWebhookEndpointUpdate =
                operationalWebhookEndpointUpdate
                ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointUpdate));

            var response = this._client.SvixHttpClient.SendRequest<OperationalWebhookEndpointOut>(
                method: HttpMethod.Put,
                path: "/api/v1/operational-webhook/endpoint/{endpoint_id}",
                pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } },
                content: operationalWebhookEndpointUpdate
            );
            return response.Data;
        }

        /// <summary>
        /// Delete an operational webhook endpoint.
        /// </summary>
        public async Task<bool> DeleteAsync(
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            var response = await this._client.SvixHttpClient.SendRequestAsync<bool>(
                method: HttpMethod.Delete,
                path: "/api/v1/operational-webhook/endpoint/{endpoint_id}",
                pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } },
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Delete an operational webhook endpoint.
        /// </summary>
        public bool Delete(string endpointId)
        {
            var response = this._client.SvixHttpClient.SendRequest<bool>(
                method: HttpMethod.Delete,
                path: "/api/v1/operational-webhook/endpoint/{endpoint_id}",
                pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } }
            );
            return response.Data;
        }

        /// <summary>
        /// Get the additional headers to be sent with the operational webhook.
        /// </summary>
        public async Task<OperationalWebhookEndpointHeadersOut> GetHeadersAsync(
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            var response =
                await this._client.SvixHttpClient.SendRequestAsync<OperationalWebhookEndpointHeadersOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
                    pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } },
                    cancellationToken: cancellationToken
                );
            return response.Data;
        }

        /// <summary>
        /// Get the additional headers to be sent with the operational webhook.
        /// </summary>
        public OperationalWebhookEndpointHeadersOut GetHeaders(string endpointId)
        {
            var response =
                this._client.SvixHttpClient.SendRequest<OperationalWebhookEndpointHeadersOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
                    pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } }
                );
            return response.Data;
        }

        /// <summary>
        /// Set the additional headers to be sent with the operational webhook.
        /// </summary>
        public async Task<bool> UpdateHeadersAsync(
            string endpointId,
            OperationalWebhookEndpointHeadersIn operationalWebhookEndpointHeadersIn,
            CancellationToken cancellationToken = default
        )
        {
            operationalWebhookEndpointHeadersIn =
                operationalWebhookEndpointHeadersIn
                ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointHeadersIn));

            var response = await this._client.SvixHttpClient.SendRequestAsync<bool>(
                method: HttpMethod.Put,
                path: "/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
                pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } },
                content: operationalWebhookEndpointHeadersIn,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Set the additional headers to be sent with the operational webhook.
        /// </summary>
        public bool UpdateHeaders(
            string endpointId,
            OperationalWebhookEndpointHeadersIn operationalWebhookEndpointHeadersIn
        )
        {
            operationalWebhookEndpointHeadersIn =
                operationalWebhookEndpointHeadersIn
                ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointHeadersIn));

            var response = this._client.SvixHttpClient.SendRequest<bool>(
                method: HttpMethod.Put,
                path: "/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
                pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } },
                content: operationalWebhookEndpointHeadersIn
            );
            return response.Data;
        }

        /// <summary>
        /// Get an operational webhook endpoint's signing secret.
        ///
        /// This is used to verify the authenticity of the webhook.
        /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
        /// </summary>
        public async Task<OperationalWebhookEndpointSecretOut> GetSecretAsync(
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            var response =
                await this._client.SvixHttpClient.SendRequestAsync<OperationalWebhookEndpointSecretOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/operational-webhook/endpoint/{endpoint_id}/secret",
                    pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } },
                    cancellationToken: cancellationToken
                );
            return response.Data;
        }

        /// <summary>
        /// Get an operational webhook endpoint's signing secret.
        ///
        /// This is used to verify the authenticity of the webhook.
        /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
        /// </summary>
        public OperationalWebhookEndpointSecretOut GetSecret(string endpointId)
        {
            var response =
                this._client.SvixHttpClient.SendRequest<OperationalWebhookEndpointSecretOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/operational-webhook/endpoint/{endpoint_id}/secret",
                    pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } }
                );
            return response.Data;
        }

        /// <summary>
        /// Rotates an operational webhook endpoint's signing secret.
        ///
        /// The previous secret will remain valid for the next 24 hours.
        /// </summary>
        public async Task<bool> RotateSecretAsync(
            string endpointId,
            OperationalWebhookEndpointSecretIn operationalWebhookEndpointSecretIn,
            OperationalWebhookEndpointRotateSecretOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            operationalWebhookEndpointSecretIn =
                operationalWebhookEndpointSecretIn
                ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointSecretIn));

            var response = await this._client.SvixHttpClient.SendRequestAsync<bool>(
                method: HttpMethod.Post,
                path: "/api/v1/operational-webhook/endpoint/{endpoint_id}/secret/rotate",
                pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: operationalWebhookEndpointSecretIn,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Rotates an operational webhook endpoint's signing secret.
        ///
        /// The previous secret will remain valid for the next 24 hours.
        /// </summary>
        public bool RotateSecret(
            string endpointId,
            OperationalWebhookEndpointSecretIn operationalWebhookEndpointSecretIn,
            OperationalWebhookEndpointRotateSecretOptions? options = null
        )
        {
            operationalWebhookEndpointSecretIn =
                operationalWebhookEndpointSecretIn
                ?? throw new ArgumentNullException(nameof(operationalWebhookEndpointSecretIn));

            var response = this._client.SvixHttpClient.SendRequest<bool>(
                method: HttpMethod.Post,
                path: "/api/v1/operational-webhook/endpoint/{endpoint_id}/secret/rotate",
                pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: operationalWebhookEndpointSecretIn
            );
            return response.Data;
        }
    }
}
