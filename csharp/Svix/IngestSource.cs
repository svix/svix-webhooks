// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class IngestSourceListOptions : SvixOptionsBase
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

    public class IngestSourceCreateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class IngestSourceRotateTokenOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class IngestSource(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// List of all the organization's Ingest Sources.
        /// </summary>
        public async Task<ListResponseIngestSourceOut> ListAsync(
            IngestSourceListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<ListResponseIngestSourceOut>(
                        method: HttpMethod.Get,
                        path: "/ingest/api/v1/source",
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
        /// List of all the organization's Ingest Sources.
        /// </summary>
        public ListResponseIngestSourceOut List(IngestSourceListOptions? options = null)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ListResponseIngestSourceOut>(
                    method: HttpMethod.Get,
                    path: "/ingest/api/v1/source",
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
        /// Create Ingest Source.
        /// </summary>
        public async Task<IngestSourceOut> CreateAsync(
            IngestSourceIn ingestSourceIn,
            IngestSourceCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            ingestSourceIn =
                ingestSourceIn ?? throw new ArgumentNullException(nameof(ingestSourceIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<IngestSourceOut>(
                    method: HttpMethod.Post,
                    path: "/ingest/api/v1/source",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: ingestSourceIn,
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
        /// Create Ingest Source.
        /// </summary>
        public IngestSourceOut Create(
            IngestSourceIn ingestSourceIn,
            IngestSourceCreateOptions? options = null
        )
        {
            ingestSourceIn =
                ingestSourceIn ?? throw new ArgumentNullException(nameof(ingestSourceIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<IngestSourceOut>(
                    method: HttpMethod.Post,
                    path: "/ingest/api/v1/source",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: ingestSourceIn
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
        /// Get an Ingest Source by id or uid.
        /// </summary>
        public async Task<IngestSourceOut> GetAsync(
            string sourceId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<IngestSourceOut>(
                    method: HttpMethod.Get,
                    path: "/ingest/api/v1/source/{source_id}",
                    pathParams: new Dictionary<string, string> { { "source_id", sourceId } },
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
        /// Get an Ingest Source by id or uid.
        /// </summary>
        public IngestSourceOut Get(string sourceId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<IngestSourceOut>(
                    method: HttpMethod.Get,
                    path: "/ingest/api/v1/source/{source_id}",
                    pathParams: new Dictionary<string, string> { { "source_id", sourceId } }
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
        /// Update an Ingest Source.
        /// </summary>
        public async Task<IngestSourceOut> UpdateAsync(
            string sourceId,
            IngestSourceIn ingestSourceIn,
            CancellationToken cancellationToken = default
        )
        {
            ingestSourceIn =
                ingestSourceIn ?? throw new ArgumentNullException(nameof(ingestSourceIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<IngestSourceOut>(
                    method: HttpMethod.Put,
                    path: "/ingest/api/v1/source/{source_id}",
                    pathParams: new Dictionary<string, string> { { "source_id", sourceId } },
                    content: ingestSourceIn,
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
        /// Update an Ingest Source.
        /// </summary>
        public IngestSourceOut Update(string sourceId, IngestSourceIn ingestSourceIn)
        {
            ingestSourceIn =
                ingestSourceIn ?? throw new ArgumentNullException(nameof(ingestSourceIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<IngestSourceOut>(
                    method: HttpMethod.Put,
                    path: "/ingest/api/v1/source/{source_id}",
                    pathParams: new Dictionary<string, string> { { "source_id", sourceId } },
                    content: ingestSourceIn
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
        /// Delete an Ingest Source.
        /// </summary>
        public async Task<bool> DeleteAsync(
            string sourceId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Delete,
                    path: "/ingest/api/v1/source/{source_id}",
                    pathParams: new Dictionary<string, string> { { "source_id", sourceId } },
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
        /// Delete an Ingest Source.
        /// </summary>
        public bool Delete(string sourceId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Delete,
                    path: "/ingest/api/v1/source/{source_id}",
                    pathParams: new Dictionary<string, string> { { "source_id", sourceId } }
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
        /// Rotate the Ingest Source's Url Token.
        ///
        /// This will rotate the ingest source's token, which is used to
        /// construct the unique `ingestUrl` for the source. Previous tokens
        /// will remain valid for 48 hours after rotation. The token can be
        /// rotated a maximum of three times within the 48-hour period.
        /// </summary>
        public async Task<RotateTokenOut> RotateTokenAsync(
            string sourceId,
            IngestSourceRotateTokenOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<RotateTokenOut>(
                    method: HttpMethod.Post,
                    path: "/ingest/api/v1/source/{source_id}/token/rotate",
                    pathParams: new Dictionary<string, string> { { "source_id", sourceId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(RotateTokenAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Rotate the Ingest Source's Url Token.
        ///
        /// This will rotate the ingest source's token, which is used to
        /// construct the unique `ingestUrl` for the source. Previous tokens
        /// will remain valid for 48 hours after rotation. The token can be
        /// rotated a maximum of three times within the 48-hour period.
        /// </summary>
        public RotateTokenOut RotateToken(
            string sourceId,
            IngestSourceRotateTokenOptions? options = null
        )
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<RotateTokenOut>(
                    method: HttpMethod.Post,
                    path: "/ingest/api/v1/source/{source_id}/token/rotate",
                    pathParams: new Dictionary<string, string> { { "source_id", sourceId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(RotateToken)} failed");

                throw;
            }
        }
    }
}
