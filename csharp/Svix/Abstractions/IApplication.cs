using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Svix.Model;
using Svix.Models;

namespace Svix.Abstractions
{
    public interface IApplication
    {
        List<ApplicationOut> List(ApplicationListOptions options, string idempotencyKey = default);
        
        Task<List<ApplicationOut>> ListAsync(ApplicationListOptions options, string idempotencyKey = default, CancellationToken cancellationToken = default);
    }
}