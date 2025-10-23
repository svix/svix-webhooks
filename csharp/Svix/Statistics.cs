// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
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
        ///
        /// The completed background task will return a payload like the following:
        /// ```json
        /// {
        ///   "id": "qtask_33qe39Stble9Rn3ZxFrqL5ZSsjT",
        ///   "status": "finished",
        ///   "task": "application.stats",
        ///   "data": {
        ///     "appStats": [
        ///       {
        ///         "messageDestinations": 2,
        ///         "appId": "app_33W1An2Zz5cO9SWbhHsYyDmVC6m",
        ///         "appUid": null
        ///       }
        ///     ]
        ///   }
        /// }
        /// ```
        /// </summary>
        public async Task<AppUsageStatsOut> AggregateAppStatsAsync(
            AppUsageStatsIn appUsageStatsIn,
            StatisticsAggregateAppStatsOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            appUsageStatsIn =
                appUsageStatsIn ?? throw new ArgumentNullException(nameof(appUsageStatsIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<AppUsageStatsOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/stats/usage/app",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: appUsageStatsIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(AggregateAppStatsAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Creates a background task to calculate the message destinations for all applications in the environment.
        ///
        /// Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
        /// retrieve the results of the operation.
        ///
        /// The completed background task will return a payload like the following:
        /// ```json
        /// {
        ///   "id": "qtask_33qe39Stble9Rn3ZxFrqL5ZSsjT",
        ///   "status": "finished",
        ///   "task": "application.stats",
        ///   "data": {
        ///     "appStats": [
        ///       {
        ///         "messageDestinations": 2,
        ///         "appId": "app_33W1An2Zz5cO9SWbhHsYyDmVC6m",
        ///         "appUid": null
        ///       }
        ///     ]
        ///   }
        /// }
        /// ```
        /// </summary>
        public AppUsageStatsOut AggregateAppStats(
            AppUsageStatsIn appUsageStatsIn,
            StatisticsAggregateAppStatsOptions? options = null
        )
        {
            appUsageStatsIn =
                appUsageStatsIn ?? throw new ArgumentNullException(nameof(appUsageStatsIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<AppUsageStatsOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/stats/usage/app",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: appUsageStatsIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(AggregateAppStats)} failed");

                throw;
            }
        }

        /// <summary>
        /// Creates a background task to calculate the listed event types for all apps in the organization.
        ///
        /// Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
        /// retrieve the results of the operation.
        ///
        /// The completed background task will return a payload like the following:
        /// ```json
        /// {
        ///   "id": "qtask_33qe39Stble9Rn3ZxFrqL5ZSsjT",
        ///   "status": "finished",
        ///   "task": "event-type.aggregate",
        ///   "data": {
        ///     "event_types": [
        ///       {
        ///         "appId": "app_33W1An2Zz5cO9SWbhHsYyDmVC6m",
        ///         "explicitlySubscribedEventTypes": ["user.signup", "user.deleted"],
        ///         "hasCatchAllEndpoint": false
        ///       }
        ///     ]
        ///   }
        /// }
        /// ```
        /// </summary>
        public async Task<AggregateEventTypesOut> AggregateEventTypesAsync(
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<AggregateEventTypesOut>(
                        method: HttpMethod.Put,
                        path: "/api/v1/stats/usage/event-types",
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(AggregateEventTypesAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Creates a background task to calculate the listed event types for all apps in the organization.
        ///
        /// Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
        /// retrieve the results of the operation.
        ///
        /// The completed background task will return a payload like the following:
        /// ```json
        /// {
        ///   "id": "qtask_33qe39Stble9Rn3ZxFrqL5ZSsjT",
        ///   "status": "finished",
        ///   "task": "event-type.aggregate",
        ///   "data": {
        ///     "event_types": [
        ///       {
        ///         "appId": "app_33W1An2Zz5cO9SWbhHsYyDmVC6m",
        ///         "explicitlySubscribedEventTypes": ["user.signup", "user.deleted"],
        ///         "hasCatchAllEndpoint": false
        ///       }
        ///     ]
        ///   }
        /// }
        /// ```
        /// </summary>
        public AggregateEventTypesOut AggregateEventTypes()
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<AggregateEventTypesOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/stats/usage/event-types"
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(AggregateEventTypes)} failed");

                throw;
            }
        }
    }
}
