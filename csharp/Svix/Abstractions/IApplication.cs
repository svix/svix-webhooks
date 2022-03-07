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
        
        List<ApplicationOut> List(ApplicationListOptions options, string idempotencyKey = default);
        
        Task<List<ApplicationOut>> ListAsync(ApplicationListOptions options, string idempotencyKey = default, CancellationToken cancellationToken = default);
    }
}