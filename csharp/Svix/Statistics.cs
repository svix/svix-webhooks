// this file is @generated
#nullable enable
using Svix.Models;

namespace Svix
{
    public class StatisticsAggregateAppStatsOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class Statistics(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Creates a background task to calculate the message destinations for all applications in the environment.
        ///
        /// Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
        /// retrieve the results of the operation.
        /// </summary>
        public async Task<AppUsageStatsOut> AggregateAppStatsAsync(
            AppUsageStatsIn appUsageStatsIn,
            StatisticsAggregateAppStatsOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            appUsageStatsIn =
                appUsageStatsIn ?? throw new ArgumentNullException(nameof(appUsageStatsIn));

            var response = await this._client.SvixHttpClient.SendRequestAsync<AppUsageStatsOut>(
                method: HttpMethod.Post,
                path: "/api/v1/stats/usage/app",
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: appUsageStatsIn,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Creates a background task to calculate the message destinations for all applications in the environment.
        ///
        /// Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
        /// retrieve the results of the operation.
        /// </summary>
        public AppUsageStatsOut AggregateAppStats(
            AppUsageStatsIn appUsageStatsIn,
            StatisticsAggregateAppStatsOptions? options = null
        )
        {
            appUsageStatsIn =
                appUsageStatsIn ?? throw new ArgumentNullException(nameof(appUsageStatsIn));

            var response = this._client.SvixHttpClient.SendRequest<AppUsageStatsOut>(
                method: HttpMethod.Post,
                path: "/api/v1/stats/usage/app",
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: appUsageStatsIn
            );
            return response.Data;
        }

        /// <summary>
        /// Creates a background task to calculate the listed event types for all apps in the organization.
        ///
        /// Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
        /// retrieve the results of the operation.
        /// </summary>
        public async Task<AggregateEventTypesOut> AggregateEventTypesAsync(
            CancellationToken cancellationToken = default
        )
        {
            var response =
                await this._client.SvixHttpClient.SendRequestAsync<AggregateEventTypesOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/stats/usage/event-types",
                    cancellationToken: cancellationToken
                );
            return response.Data;
        }

        /// <summary>
        /// Creates a background task to calculate the listed event types for all apps in the organization.
        ///
        /// Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
        /// retrieve the results of the operation.
        /// </summary>
        public AggregateEventTypesOut AggregateEventTypes()
        {
            var response = this._client.SvixHttpClient.SendRequest<AggregateEventTypesOut>(
                method: HttpMethod.Put,
                path: "/api/v1/stats/usage/event-types"
            );
            return response.Data;
        }
    }
}
