using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Svix.Model;
using Svix.Models;

namespace Svix.Abstractions
{
    public interface IStatistics
    {
        AppUsageStatsOut AggregateAppStats(AppUsageStatsIn appUsageStatsIn, string idempotencyKey = default);

        Task<AppUsageStatsOut> AggregateAppStatsAsync(AppUsageStatsIn appUsageStatsIn, string idempotencyKey = default);

        AggregateEventTypesOut AggregateEventTypes();

        Task<AggregateEventTypesOut> AggregateEventTypesAsync();

    }
}
