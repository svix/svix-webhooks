using System.Threading;
using System.Threading.Tasks;

namespace Svix.Abstractions
{
    public interface ISvixClient
    {
        #region " Health "
        
        bool IsHealthy(string idempotencyKey = default);

        Task<bool> IsHealthyAsync(string idempotencyKey = default, CancellationToken cancellationToken = default);
        
        #endregion
    }
}