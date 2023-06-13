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

        List<BackgroundTaskOut> List(BackgroundTaskListOptions options = null, string idempotencyKey = default);

        Task<List<BackgroundTaskOut>> ListAsync(BackgroundTaskListOptions options = null, string idempotencyKey = default, CancellationToken cancellationToken = default);
    }
}
