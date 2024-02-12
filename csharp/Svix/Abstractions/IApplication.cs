using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Svix.Model;
using Svix.Models;

namespace Svix.Abstractions
{
    public interface IApplication
    {
        public ApplicationOut Create(ApplicationIn application, ApplicationCreateOptions options = null,
            string idempotencyKey = default);

        public Task<ApplicationOut> CreateAsync(ApplicationIn application, ApplicationCreateOptions options = null,
            string idempotencyKey = default, CancellationToken cancellationToken = default);

        bool Delete(string appId, string idempotencyKey = default);

        Task<bool> DeleteAsync(string appId, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        ApplicationOut Get(string appId, string idempotencyKey = default);

        Task<ApplicationOut> GetAsync(string appId, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        ListResponseApplicationOut List(ListOptions options = null, string idempotencyKey = default);

        Task<ListResponseApplicationOut> ListAsync(ListOptions options = null, string idempotencyKey = default, CancellationToken cancellationToken = default);

        ApplicationOut Update(string appId, ApplicationIn application, string idempotencyKey = default);

        Task<ApplicationOut> UpdateAsync(string appId, ApplicationIn application, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        ApplicationOut Patch(string appId, ApplicationPatch application, string idempotencyKey = default);

        Task<ApplicationOut> PatchAsync(string appId, ApplicationPatch application, string idempotencyKey = default,
            CancellationToken cancellationToken = default);
    }
}
