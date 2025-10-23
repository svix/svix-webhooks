// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class EnvironmentExportOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class EnvironmentImportOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class Environment(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Download a JSON file containing all org-settings and event types.
        ///
        /// Note that the schema for [`EnvironmentOut`] is subject to change. The fields
        /// herein are provided for convenience but should be treated as JSON blobs.
        /// </summary>
        public async Task<EnvironmentOut> ExportAsync(
            EnvironmentExportOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<EnvironmentOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/environment/export",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ExportAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Download a JSON file containing all org-settings and event types.
        ///
        /// Note that the schema for [`EnvironmentOut`] is subject to change. The fields
        /// herein are provided for convenience but should be treated as JSON blobs.
        /// </summary>
        public EnvironmentOut Export(EnvironmentExportOptions? options = null)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<EnvironmentOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/environment/export",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Export)} failed");

                throw;
            }
        }

        /// <summary>
        /// Import a configuration into the active organization.
        ///
        /// It doesn't delete anything, only adds / updates what was passed to it.
        ///
        /// Note that the schema for [`EnvironmentIn`] is subject to change. The fields
        /// herein are provided for convenience but should be treated as JSON blobs.
        /// </summary>
        public async Task<bool> ImportAsync(
            EnvironmentIn environmentIn,
            EnvironmentImportOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            environmentIn = environmentIn ?? throw new ArgumentNullException(nameof(environmentIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Post,
                    path: "/api/v1/environment/import",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: environmentIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ImportAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Import a configuration into the active organization.
        ///
        /// It doesn't delete anything, only adds / updates what was passed to it.
        ///
        /// Note that the schema for [`EnvironmentIn`] is subject to change. The fields
        /// herein are provided for convenience but should be treated as JSON blobs.
        /// </summary>
        public bool Import(EnvironmentIn environmentIn, EnvironmentImportOptions? options = null)
        {
            environmentIn = environmentIn ?? throw new ArgumentNullException(nameof(environmentIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Post,
                    path: "/api/v1/environment/import",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: environmentIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Import)} failed");

                throw;
            }
        }
    }
}
