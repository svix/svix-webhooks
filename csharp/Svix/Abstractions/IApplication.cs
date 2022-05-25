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

        List<ApplicationOut> List(ListOptions options = null, string idempotencyKey = default);

        Task<List<ApplicationOut>> ListAsync(ListOptions options = null, string idempotencyKey = default, CancellationToken cancellationToken = default);

        ApplicationOut Update(string appId, ApplicationIn application, string idempotencyKey = default);

        Task<ApplicationOut> UpdateAsync(string appId, ApplicationIn application, string idempotencyKey = default,
            CancellationToken cancellationToken = default);
    }
}
