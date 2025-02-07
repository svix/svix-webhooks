using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Svix.Model;
using Svix.Models;

namespace Svix.Abstractions
{
    public interface IBackgroundTask
    {
        BackgroundTaskOut Get(string taskId, string idempotencyKey = default);

        Task<BackgroundTaskOut> GetAsync(string taskId, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        ListResponseBackgroundTaskOut List(BackgroundTaskListOptions options = null, string idempotencyKey = default);

        Task<ListResponseBackgroundTaskOut> ListAsync(BackgroundTaskListOptions options = null, string idempotencyKey = default, CancellationToken cancellationToken = default);
    }
}
