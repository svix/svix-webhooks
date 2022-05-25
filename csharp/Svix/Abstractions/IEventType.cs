using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Svix.Model;
using Svix.Models;

namespace Svix.Abstractions
{
    public interface IEventType
    {
        bool Archive(string eventType, string idempotencyKey = default);

        Task<bool> ArchiveAsync(string eventType, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        EventTypeOut Create(EventTypeIn eventType, string idempotencyKey = default);

        Task<EventTypeOut> CreateAsync(EventTypeIn eventType, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        EventTypeOut Get(string eventType, string idempotencyKey = default);

        Task<EventTypeOut> GetAsync(string eventType, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        List<EventTypeOut> List(EventTypeListOptions options = null, string idempotencyKey = default);

        Task<List<EventTypeOut>> ListAsync(EventTypeListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        EventTypeOut Update(string eventType, EventTypeUpdate update, string idempotencyKey = default);

        Task<EventTypeOut> UpdateAsync(string eventType, EventTypeUpdate update, string idempotencyKey = default,
            CancellationToken cancellationToken = default);
    }
}
