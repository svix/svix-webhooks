// this file is @generated
#nullable enable
using System;
using System.Collections.Generic;
using System.Net;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;
using Svix.Abstractions;
using Svix.Api;
using Svix.Client;
using Svix.Model;
using Svix.Models;

namespace Svix
{
    public partial class StatisticsAggregateAppStatsOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public sealed class Statistics : SvixResourceBase
    {
        private readonly StatisticsApi _statisticsApi;

        public Statistics(ISvixClient svixClient, StatisticsApi statisticsApi)
            : base(svixClient)
        {
            _statisticsApi =
                statisticsApi ?? throw new ArgumentNullException(nameof(statisticsApi));
        }

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
            try
            {
                appUsageStatsIn =
                    appUsageStatsIn ?? throw new ArgumentNullException(nameof(appUsageStatsIn));
                var response = await _statisticsApi.V1StatisticsAggregateAppStatsWithHttpInfoAsync(
                    appUsageStatsIn: appUsageStatsIn,
                    idempotencyKey: options?.IdempotencyKey,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(AggregateAppStatsAsync)} failed");

                if (Throw)
                    throw;
                return new AppUsageStatsOut();
            }
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
            try
            {
                appUsageStatsIn =
                    appUsageStatsIn ?? throw new ArgumentNullException(nameof(appUsageStatsIn));
                var response = _statisticsApi.V1StatisticsAggregateAppStatsWithHttpInfo(
                    appUsageStatsIn: appUsageStatsIn,
                    idempotencyKey: options?.IdempotencyKey
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(AggregateAppStats)} failed");

                if (Throw)
                    throw;
                return new AppUsageStatsOut();
            }
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
            try
            {
                var response =
                    await _statisticsApi.V1StatisticsAggregateEventTypesWithHttpInfoAsync(
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(AggregateEventTypesAsync)} failed");

                if (Throw)
                    throw;
                return new AggregateEventTypesOut();
            }
        }

        /// <summary>
        /// Creates a background task to calculate the listed event types for all apps in the organization.
        ///
        /// Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
        /// retrieve the results of the operation.
        /// </summary>
        public AggregateEventTypesOut AggregateEventTypes()
        {
            try
            {
                var response = _statisticsApi.V1StatisticsAggregateEventTypesWithHttpInfo();
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(AggregateEventTypes)} failed");

                if (Throw)
                    throw;
                return new AggregateEventTypesOut();
            }
        }
    }
}
