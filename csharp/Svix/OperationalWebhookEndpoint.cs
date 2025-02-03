// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
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
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<ListResponseOperationalWebhookEndpointOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/operational-webhook/endpoint",
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
        /// List operational webhook endpoints.
        /// </summary>
        public ListResponseOperationalWebhookEndpointOut List(
            OperationalWebhookEndpointListOptions? options = null
        )
        {
            try
            {
                var response =
                    _client.SvixHttpClient.SendRequest<ListResponseOperationalWebhookEndpointOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/operational-webhook/endpoint",
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
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<OperationalWebhookEndpointOut>(
                        method: HttpMethod.Post,
                        path: "/api/v1/operational-webhook/endpoint",
                        queryParams: options?.QueryParams(),
                        headerParams: options?.HeaderParams(),
                        content: operationalWebhookEndpointIn,
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
            try
            {
                var response = _client.SvixHttpClient.SendRequest<OperationalWebhookEndpointOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/operational-webhook/endpoint",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: operationalWebhookEndpointIn
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
        /// Get an operational webhook endpoint.
        /// </summary>
        public async Task<OperationalWebhookEndpointOut> GetAsync(
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<OperationalWebhookEndpointOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/operational-webhook/endpoint/{endpoint_id}",
                        pathParams: new Dictionary<string, string>
                        {
                            { "endpoint_id", endpointId },
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
        /// Get an operational webhook endpoint.
        /// </summary>
        public OperationalWebhookEndpointOut Get(string endpointId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<OperationalWebhookEndpointOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/operational-webhook/endpoint/{endpoint_id}",
                    pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } }
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
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<OperationalWebhookEndpointOut>(
                        method: HttpMethod.Put,
                        path: "/api/v1/operational-webhook/endpoint/{endpoint_id}",
                        pathParams: new Dictionary<string, string>
                        {
                            { "endpoint_id", endpointId },
                        },
                        content: operationalWebhookEndpointUpdate,
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
            try
            {
                var response = _client.SvixHttpClient.SendRequest<OperationalWebhookEndpointOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/operational-webhook/endpoint/{endpoint_id}",
                    pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } },
                    content: operationalWebhookEndpointUpdate
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
        /// Delete an operational webhook endpoint.
        /// </summary>
        public async Task<bool> DeleteAsync(
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/operational-webhook/endpoint/{endpoint_id}",
                    pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } },
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
        /// Delete an operational webhook endpoint.
        /// </summary>
        public bool Delete(string endpointId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/operational-webhook/endpoint/{endpoint_id}",
                    pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } }
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
        /// Get the additional headers to be sent with the operational webhook.
        /// </summary>
        public async Task<OperationalWebhookEndpointHeadersOut> GetHeadersAsync(
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<OperationalWebhookEndpointHeadersOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
                        pathParams: new Dictionary<string, string>
                        {
                            { "endpoint_id", endpointId },
                        },
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(GetHeadersAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get the additional headers to be sent with the operational webhook.
        /// </summary>
        public OperationalWebhookEndpointHeadersOut GetHeaders(string endpointId)
        {
            try
            {
                var response =
                    _client.SvixHttpClient.SendRequest<OperationalWebhookEndpointHeadersOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
                        pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } }
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(GetHeaders)} failed");

                throw;
            }
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
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Put,
                    path: "/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
                    pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } },
                    content: operationalWebhookEndpointHeadersIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(UpdateHeadersAsync)} failed");

                throw;
            }
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
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Put,
                    path: "/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
                    pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } },
                    content: operationalWebhookEndpointHeadersIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(UpdateHeaders)} failed");

                throw;
            }
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
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<OperationalWebhookEndpointSecretOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/operational-webhook/endpoint/{endpoint_id}/secret",
                        pathParams: new Dictionary<string, string>
                        {
                            { "endpoint_id", endpointId },
                        },
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(GetSecretAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get an operational webhook endpoint's signing secret.
        ///
        /// This is used to verify the authenticity of the webhook.
        /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
        /// </summary>
        public OperationalWebhookEndpointSecretOut GetSecret(string endpointId)
        {
            try
            {
                var response =
                    _client.SvixHttpClient.SendRequest<OperationalWebhookEndpointSecretOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/operational-webhook/endpoint/{endpoint_id}/secret",
                        pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } }
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(GetSecret)} failed");

                throw;
            }
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
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
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
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(RotateSecretAsync)} failed");

                throw;
            }
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
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Post,
                    path: "/api/v1/operational-webhook/endpoint/{endpoint_id}/secret/rotate",
                    pathParams: new Dictionary<string, string> { { "endpoint_id", endpointId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: operationalWebhookEndpointSecretIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(RotateSecret)} failed");

                throw;
            }
        }
    }
}
