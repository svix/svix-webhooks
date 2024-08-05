using System.Threading;
using System.Threading.Tasks;
using Svix.Model;

namespace Svix.Abstractions
{
    public interface IAuthentication
    {
        AppPortalAccessOut GetAppPortalAccess(string appId, AppPortalAccessIn appPortalAccess, string idempotencyKey = default);

        Task<AppPortalAccessOut> GetAppPortalAccessAsync(string appId, AppPortalAccessIn appPortalAccess,
            string idempotencyKey = default, CancellationToken cancellationToken = default);

        DashboardAccessOut GetDashboardAccess(string appId, string idempotencyKey = default);

        Task<DashboardAccessOut> GetDashboardAccessAsync(string appId, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        bool Logout(string idempotencyKey = default);

        Task<bool> LogoutAsync(string idempotencyKey = default, CancellationToken cancellationToken = default);
    }
}