// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class IngestAuthenticationConsumerPortalAccessOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class IngestAuthentication(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Get access to the Ingest Source Consumer Portal.
        /// </summary>
        public async Task<AppPortalAccessOut> ConsumerPortalAccessAsync(
            string sourceId,
            IngestSourceConsumerPortalAccessIn ingestSourceConsumerPortalAccessIn,
            IngestAuthenticationConsumerPortalAccessOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            if (options == null)
            {
                options = new IngestAuthenticationConsumerPortalAccessOptions();
            }
            ingestSourceConsumerPortalAccessIn =
                ingestSourceConsumerPortalAccessIn
                ?? throw new ArgumentNullException(nameof(ingestSourceConsumerPortalAccessIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<AppPortalAccessOut>(
                    method: HttpMethod.Post,
                    path: "/ingest/api/v1/source/{source_id}/dashboard",
                    pathParams: new Dictionary<string, string> { { "source_id", sourceId } },
                    queryParams: options.QueryParams(),
                    headerParams: options.HeaderParams(),
                    content: ingestSourceConsumerPortalAccessIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ConsumerPortalAccessAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get access to the Ingest Source Consumer Portal.
        /// </summary>
        public AppPortalAccessOut ConsumerPortalAccess(
            string sourceId,
            IngestSourceConsumerPortalAccessIn ingestSourceConsumerPortalAccessIn,
            IngestAuthenticationConsumerPortalAccessOptions? options = null
        )
        {
            if (options == null)
            {
                options = new IngestAuthenticationConsumerPortalAccessOptions();
            }
            ingestSourceConsumerPortalAccessIn =
                ingestSourceConsumerPortalAccessIn
                ?? throw new ArgumentNullException(nameof(ingestSourceConsumerPortalAccessIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<AppPortalAccessOut>(
                    method: HttpMethod.Post,
                    path: "/ingest/api/v1/source/{source_id}/dashboard",
                    pathParams: new Dictionary<string, string> { { "source_id", sourceId } },
                    queryParams: options.QueryParams(),
                    headerParams: options.HeaderParams(),
                    content: ingestSourceConsumerPortalAccessIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ConsumerPortalAccess)} failed");

                throw;
            }
        }
    }
}
