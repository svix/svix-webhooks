using System;
using System.Net;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;
using Svix.Abstractions;
using Svix.Api;
using Svix.Client;
using Svix.Model;

namespace Svix
{
    public sealed class Authentication : SvixResourceBase, IAuthentication
    {
        private readonly IAuthenticationApi _authenticationApi;

        public Authentication(ISvixClient svixClient, IAuthenticationApi authenticationApi)
            : base(svixClient)
        {
            _authenticationApi = authenticationApi ?? throw new ArgumentNullException(nameof(authenticationApi));
        }

        public AppPortalAccessOut GetAppPortalAccess(string appId, AppPortalAccessIn appPortalAccess, string idempotencyKey = default)
        {
            try
            {
                var lMessage = _authenticationApi.GetAppPortalAccessApiV1AuthAppPortalAccessAppIdPost(
                    appId,
                    appPortalAccess,
                    idempotencyKey);

                return lMessage;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAppPortalAccess)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<AppPortalAccessOut> GetAppPortalAccessAsync(string appId, AppPortalAccessIn appPortalAccess,
            string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lMessage = await _authenticationApi.GetAppPortalAccessApiV1AuthAppPortalAccessAppIdPostAsync(
                    appId,
                    appPortalAccess,
                    idempotencyKey);

                return lMessage;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAppPortalAccessAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public DashboardAccessOut GetDashboardAccess(string appId, string idempotencyKey = default)
        {
            try
            {
                var lMessage = _authenticationApi.GetDashboardAccessApiV1AuthDashboardAccessAppIdPost(
                    appId,
                    idempotencyKey);

                return lMessage;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetDashboardAccess)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<DashboardAccessOut> GetDashboardAccessAsync(string appId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lMessage = await _authenticationApi.GetDashboardAccessApiV1AuthDashboardAccessAppIdPostAsync(
                    appId,
                    idempotencyKey);

                return lMessage;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetDashboardAccessAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public bool Logout(string idempotencyKey = default)
        {
            try
            {
                var lResult = _authenticationApi.LogoutApiV1AuthLogoutPostWithHttpInfo(
                    idempotencyKey);

                return lResult.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Logout)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public async Task<bool> LogoutAsync(string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResult = await _authenticationApi.LogoutApiV1AuthLogoutPostWithHttpInfoAsync(
                    idempotencyKey,
                    cancellationToken);

                return lResult.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(LogoutAsync)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }
    }
}