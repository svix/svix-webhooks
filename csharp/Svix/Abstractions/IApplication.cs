using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Svix.Model;
using Svix.Models;

namespace Svix.Abstractions
{
    public interface IApplication
    {
        public ApplicationOut Create(ApplicationIn application, ApplicationCreateOptions options,
            string idempotencyKey = default);

        public Task<ApplicationOut> CreateAsync(ApplicationIn application, ApplicationCreateOptions options,
            string idempotencyKey = default, CancellationToken cancellationToken = default);

        bool Delete(string appId, string idempotencyKey = default);
            
        Task<bool> DeleteAsync(string appId, string idempotencyKey = default,
            CancellationToken cancellationToken = default);
        
        List<ApplicationOut> List(ApplicationListOptions options, string idempotencyKey = default);
        
        Task<List<ApplicationOut>> ListAsync(ApplicationListOptions options, string idempotencyKey = default, CancellationToken cancellationToken = default);

        ApplicationOut Update(string appId, ApplicationIn application, string idempotencyKey = default);
        
        Task<ApplicationOut> UpdateAsync(string appId, ApplicationIn application, string idempotencyKey = default,
            CancellationToken cancellationToken = default);
    }
}