// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class IngestDashboardOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class Ingest(SvixClient client)
    {
        readonly SvixClient _client = client;

        public IngestEndpoint Endpoint
        {
            get => new IngestEndpoint(_client);
        }

        public IngestSource Source
        {
            get => new IngestSource(_client);
        }

        /// <summary>
        /// Get access to the Ingest Source Consumer Portal.
        /// </summary>
        public async Task<DashboardAccessOut> DashboardAsync(
            string sourceId,
            IngestSourceConsumerPortalAccessIn ingestSourceConsumerPortalAccessIn,
            IngestDashboardOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            ingestSourceConsumerPortalAccessIn =
                ingestSourceConsumerPortalAccessIn
                ?? throw new ArgumentNullException(nameof(ingestSourceConsumerPortalAccessIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<DashboardAccessOut>(
                    method: HttpMethod.Post,
                    path: "/ingest/api/v1/source/{source_id}/dashboard",
                    pathParams: new Dictionary<string, string> { { "source_id", sourceId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: ingestSourceConsumerPortalAccessIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(DashboardAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get access to the Ingest Source Consumer Portal.
        /// </summary>
        public DashboardAccessOut Dashboard(
            string sourceId,
            IngestSourceConsumerPortalAccessIn ingestSourceConsumerPortalAccessIn,
            IngestDashboardOptions? options = null
        )
        {
            ingestSourceConsumerPortalAccessIn =
                ingestSourceConsumerPortalAccessIn
                ?? throw new ArgumentNullException(nameof(ingestSourceConsumerPortalAccessIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<DashboardAccessOut>(
                    method: HttpMethod.Post,
                    path: "/ingest/api/v1/source/{source_id}/dashboard",
                    pathParams: new Dictionary<string, string> { { "source_id", sourceId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: ingestSourceConsumerPortalAccessIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Dashboard)} failed");

                throw;
            }
        }
    }
}
