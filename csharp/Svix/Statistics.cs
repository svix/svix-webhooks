using System;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;
using Svix.Abstractions;
using Svix.Api;
using Svix.Client;
using Svix.Model;

namespace Svix
{
    public sealed class Statistics : SvixResourceBase, IStatistics
    {
        private readonly IStatisticsApi _statisticsApi;

        public Statistics(ISvixClient svixClient, IStatisticsApi statisticsApi)
            : base(svixClient)
        {
            _statisticsApi = statisticsApi ?? throw new ArgumentNullException(nameof(statisticsApi));
        }

        public AppUsageStatsOut CalculateAggregateAppStats(AppUsageStatsIn appUsageStatsIn, string idempotencyKey = default)
        {
            try
            {
                var res = _statisticsApi.CalculateAggregateAppStats(
                    appUsageStatsIn,
                    idempotencyKey);

                return res;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CalculateAggregateAppStats)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<AppUsageStatsOut> CalculateAggregateAppStatsAsync(AppUsageStatsIn appUsageStatsIn, string idempotencyKey = default)
        {
            try
            {
                var res = await _statisticsApi.CalculateAggregateAppStatsAsync(
                    appUsageStatsIn,
                    idempotencyKey);

                return res;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CalculateAggregateAppStatsAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public AggregateEventTypesOut AggregateEventTypes()
        {
            try
            {
                var res = _statisticsApi.AggregateEventTypes();

                return res;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CalculateAggregateAppStatsAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<AggregateEventTypesOut> AggregateEventTypesAsync()
        {
            try
            {
                var res = await _statisticsApi.AggregateEventTypesAsync();

                return res;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CalculateAggregateAppStatsAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }
    }
}
