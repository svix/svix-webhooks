// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class IngestEndpointListOptions : SvixOptionsBase
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

    public class IngestEndpointCreateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class IngestEndpointRotateSecretOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class IngestEndpoint(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// List ingest endpoints.
        /// </summary>
        public async Task<ListResponseIngestEndpointOut> ListAsync(
            string sourceId,
            IngestEndpointListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<ListResponseIngestEndpointOut>(
                        method: HttpMethod.Get,
                        path: "/ingest/api/v1/source/{source_id}/endpoint",
                        pathParams: new Dictionary<string, string> { { "source_id", sourceId } },
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
        /// List ingest endpoints.
        /// </summary>
        public ListResponseIngestEndpointOut List(
            string sourceId,
            IngestEndpointListOptions? options = null
        )
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ListResponseIngestEndpointOut>(
                    method: HttpMethod.Get,
                    path: "/ingest/api/v1/source/{source_id}/endpoint",
                    pathParams: new Dictionary<string, string> { { "source_id", sourceId } },
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
        /// Create an ingest endpoint.
        /// </summary>
        public async Task<IngestEndpointOut> CreateAsync(
            string sourceId,
            IngestEndpointIn ingestEndpointIn,
            IngestEndpointCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            ingestEndpointIn =
                ingestEndpointIn ?? throw new ArgumentNullException(nameof(ingestEndpointIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<IngestEndpointOut>(
                    method: HttpMethod.Post,
                    path: "/ingest/api/v1/source/{source_id}/endpoint",
                    pathParams: new Dictionary<string, string> { { "source_id", sourceId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: ingestEndpointIn,
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
        /// Create an ingest endpoint.
        /// </summary>
        public IngestEndpointOut Create(
            string sourceId,
            IngestEndpointIn ingestEndpointIn,
            IngestEndpointCreateOptions? options = null
        )
        {
            ingestEndpointIn =
                ingestEndpointIn ?? throw new ArgumentNullException(nameof(ingestEndpointIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<IngestEndpointOut>(
                    method: HttpMethod.Post,
                    path: "/ingest/api/v1/source/{source_id}/endpoint",
                    pathParams: new Dictionary<string, string> { { "source_id", sourceId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: ingestEndpointIn
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
        /// Get an ingest endpoint.
        /// </summary>
        public async Task<IngestEndpointOut> GetAsync(
            string sourceId,
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<IngestEndpointOut>(
                    method: HttpMethod.Get,
                    path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "source_id", sourceId },
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
        /// Get an ingest endpoint.
        /// </summary>
        public IngestEndpointOut Get(string sourceId, string endpointId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<IngestEndpointOut>(
                    method: HttpMethod.Get,
                    path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "source_id", sourceId },
                        { "endpoint_id", endpointId },
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
        /// Update an ingest endpoint.
        /// </summary>
        public async Task<IngestEndpointOut> UpdateAsync(
            string sourceId,
            string endpointId,
            IngestEndpointUpdate ingestEndpointUpdate,
            CancellationToken cancellationToken = default
        )
        {
            ingestEndpointUpdate =
                ingestEndpointUpdate
                ?? throw new ArgumentNullException(nameof(ingestEndpointUpdate));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<IngestEndpointOut>(
                    method: HttpMethod.Put,
                    path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "source_id", sourceId },
                        { "endpoint_id", endpointId },
                    },
                    content: ingestEndpointUpdate,
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
        /// Update an ingest endpoint.
        /// </summary>
        public IngestEndpointOut Update(
            string sourceId,
            string endpointId,
            IngestEndpointUpdate ingestEndpointUpdate
        )
        {
            ingestEndpointUpdate =
                ingestEndpointUpdate
                ?? throw new ArgumentNullException(nameof(ingestEndpointUpdate));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<IngestEndpointOut>(
                    method: HttpMethod.Put,
                    path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "source_id", sourceId },
                        { "endpoint_id", endpointId },
                    },
                    content: ingestEndpointUpdate
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
        /// Delete an ingest endpoint.
        /// </summary>
        public async Task<bool> DeleteAsync(
            string sourceId,
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Delete,
                    path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "source_id", sourceId },
                        { "endpoint_id", endpointId },
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
        /// Delete an ingest endpoint.
        /// </summary>
        public bool Delete(string sourceId, string endpointId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Delete,
                    path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "source_id", sourceId },
                        { "endpoint_id", endpointId },
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
        /// Get the additional headers to be sent with the ingest.
        /// </summary>
        public async Task<IngestEndpointHeadersOut> GetHeadersAsync(
            string sourceId,
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<IngestEndpointHeadersOut>(
                        method: HttpMethod.Get,
                        path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/headers",
                        pathParams: new Dictionary<string, string>
                        {
                            { "source_id", sourceId },
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
        /// Get the additional headers to be sent with the ingest.
        /// </summary>
        public IngestEndpointHeadersOut GetHeaders(string sourceId, string endpointId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<IngestEndpointHeadersOut>(
                    method: HttpMethod.Get,
                    path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/headers",
                    pathParams: new Dictionary<string, string>
                    {
                        { "source_id", sourceId },
                        { "endpoint_id", endpointId },
                    }
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
        /// Set the additional headers to be sent to the endpoint.
        /// </summary>
        public async Task<bool> UpdateHeadersAsync(
            string sourceId,
            string endpointId,
            IngestEndpointHeadersIn ingestEndpointHeadersIn,
            CancellationToken cancellationToken = default
        )
        {
            ingestEndpointHeadersIn =
                ingestEndpointHeadersIn
                ?? throw new ArgumentNullException(nameof(ingestEndpointHeadersIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Put,
                    path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/headers",
                    pathParams: new Dictionary<string, string>
                    {
                        { "source_id", sourceId },
                        { "endpoint_id", endpointId },
                    },
                    content: ingestEndpointHeadersIn,
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
        /// Set the additional headers to be sent to the endpoint.
        /// </summary>
        public bool UpdateHeaders(
            string sourceId,
            string endpointId,
            IngestEndpointHeadersIn ingestEndpointHeadersIn
        )
        {
            ingestEndpointHeadersIn =
                ingestEndpointHeadersIn
                ?? throw new ArgumentNullException(nameof(ingestEndpointHeadersIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Put,
                    path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/headers",
                    pathParams: new Dictionary<string, string>
                    {
                        { "source_id", sourceId },
                        { "endpoint_id", endpointId },
                    },
                    content: ingestEndpointHeadersIn
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
        /// Get an ingest endpoint's signing secret.
        ///
        /// This is used to verify the authenticity of the webhook.
        /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
        /// </summary>
        public async Task<IngestEndpointSecretOut> GetSecretAsync(
            string sourceId,
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<IngestEndpointSecretOut>(
                        method: HttpMethod.Get,
                        path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/secret",
                        pathParams: new Dictionary<string, string>
                        {
                            { "source_id", sourceId },
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
        /// Get an ingest endpoint's signing secret.
        ///
        /// This is used to verify the authenticity of the webhook.
        /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
        /// </summary>
        public IngestEndpointSecretOut GetSecret(string sourceId, string endpointId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<IngestEndpointSecretOut>(
                    method: HttpMethod.Get,
                    path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/secret",
                    pathParams: new Dictionary<string, string>
                    {
                        { "source_id", sourceId },
                        { "endpoint_id", endpointId },
                    }
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
        /// Rotates an ingest endpoint's signing secret.
        ///
        /// The previous secret will remain valid for the next 24 hours.
        /// </summary>
        public async Task<bool> RotateSecretAsync(
            string sourceId,
            string endpointId,
            IngestEndpointSecretIn ingestEndpointSecretIn,
            IngestEndpointRotateSecretOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            ingestEndpointSecretIn =
                ingestEndpointSecretIn
                ?? throw new ArgumentNullException(nameof(ingestEndpointSecretIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Post,
                    path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/secret/rotate",
                    pathParams: new Dictionary<string, string>
                    {
                        { "source_id", sourceId },
                        { "endpoint_id", endpointId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: ingestEndpointSecretIn,
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
        /// Rotates an ingest endpoint's signing secret.
        ///
        /// The previous secret will remain valid for the next 24 hours.
        /// </summary>
        public bool RotateSecret(
            string sourceId,
            string endpointId,
            IngestEndpointSecretIn ingestEndpointSecretIn,
            IngestEndpointRotateSecretOptions? options = null
        )
        {
            ingestEndpointSecretIn =
                ingestEndpointSecretIn
                ?? throw new ArgumentNullException(nameof(ingestEndpointSecretIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Post,
                    path: "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/secret/rotate",
                    pathParams: new Dictionary<string, string>
                    {
                        { "source_id", sourceId },
                        { "endpoint_id", endpointId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: ingestEndpointSecretIn
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
