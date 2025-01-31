// this file is @generated
#nullable enable
using System;
using System.Collections.Generic;
using System.Net;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;
using Svix.Abstractions;
using Svix.Api;
using Svix.Client;
using Svix.Model;
using Svix.Models;

namespace Svix
{
    public partial class AuthenticationAppPortalAccessOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public partial class AuthenticationExpireAllOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public partial class AuthenticationDashboardAccessOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public partial class AuthenticationLogoutOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public sealed class Authentication : SvixResourceBase
    {
        private readonly AuthenticationApi _authenticationApi;

        public Authentication(ISvixClient svixClient, AuthenticationApi authenticationApi)
            : base(svixClient)
        {
            _authenticationApi =
                authenticationApi ?? throw new ArgumentNullException(nameof(authenticationApi));
        }

        /// <summary>
        /// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
        /// </summary>
        public async Task<AppPortalAccessOut> AppPortalAccessAsync(
            string appId,
            AppPortalAccessIn appPortalAccessIn,
            AuthenticationAppPortalAccessOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                appPortalAccessIn =
                    appPortalAccessIn ?? throw new ArgumentNullException(nameof(appPortalAccessIn));
                var response =
                    await _authenticationApi.V1AuthenticationAppPortalAccessWithHttpInfoAsync(
                        appId: appId,
                        appPortalAccessIn: appPortalAccessIn,
                        idempotencyKey: options?.IdempotencyKey,
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(AppPortalAccessAsync)} failed");

                if (Throw)
                    throw;
                return new AppPortalAccessOut();
            }
        }

        /// <summary>
        /// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
        /// </summary>
        public AppPortalAccessOut AppPortalAccess(
            string appId,
            AppPortalAccessIn appPortalAccessIn,
            AuthenticationAppPortalAccessOptions? options = null
        )
        {
            try
            {
                appPortalAccessIn =
                    appPortalAccessIn ?? throw new ArgumentNullException(nameof(appPortalAccessIn));
                var response = _authenticationApi.V1AuthenticationAppPortalAccessWithHttpInfo(
                    appId: appId,
                    appPortalAccessIn: appPortalAccessIn,
                    idempotencyKey: options?.IdempotencyKey
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(AppPortalAccess)} failed");

                if (Throw)
                    throw;
                return new AppPortalAccessOut();
            }
        }

        /// <summary>
        /// Expire all of the tokens associated with a specific application.
        /// </summary>
        public async Task<bool> ExpireAllAsync(
            string appId,
            ApplicationTokenExpireIn applicationTokenExpireIn,
            AuthenticationExpireAllOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                applicationTokenExpireIn =
                    applicationTokenExpireIn
                    ?? throw new ArgumentNullException(nameof(applicationTokenExpireIn));
                var response = await _authenticationApi.V1AuthenticationExpireAllWithHttpInfoAsync(
                    appId: appId,
                    applicationTokenExpireIn: applicationTokenExpireIn,
                    idempotencyKey: options?.IdempotencyKey,
                    cancellationToken: cancellationToken
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ExpireAllAsync)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }

        /// <summary>
        /// Expire all of the tokens associated with a specific application.
        /// </summary>
        public bool ExpireAll(
            string appId,
            ApplicationTokenExpireIn applicationTokenExpireIn,
            AuthenticationExpireAllOptions? options = null
        )
        {
            try
            {
                applicationTokenExpireIn =
                    applicationTokenExpireIn
                    ?? throw new ArgumentNullException(nameof(applicationTokenExpireIn));
                var response = _authenticationApi.V1AuthenticationExpireAllWithHttpInfo(
                    appId: appId,
                    applicationTokenExpireIn: applicationTokenExpireIn,
                    idempotencyKey: options?.IdempotencyKey
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ExpireAll)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }

        /// <summary>
        /// DEPRECATED: Please use `app-portal-access` instead.
        ///
        /// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
        /// </summary>
        [Obsolete]
        public async Task<DashboardAccessOut> DashboardAccessAsync(
            string appId,
            AuthenticationDashboardAccessOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _authenticationApi.V1AuthenticationDashboardAccessWithHttpInfoAsync(
                        appId: appId,
                        idempotencyKey: options?.IdempotencyKey,
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(DashboardAccessAsync)} failed");

                if (Throw)
                    throw;
                return new DashboardAccessOut();
            }
        }

        /// <summary>
        /// DEPRECATED: Please use `app-portal-access` instead.
        ///
        /// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
        /// </summary>
        [Obsolete]
        public DashboardAccessOut DashboardAccess(
            string appId,
            AuthenticationDashboardAccessOptions? options = null
        )
        {
            try
            {
                var response = _authenticationApi.V1AuthenticationDashboardAccessWithHttpInfo(
                    appId: appId,
                    idempotencyKey: options?.IdempotencyKey
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(DashboardAccess)} failed");

                if (Throw)
                    throw;
                return new DashboardAccessOut();
            }
        }

        /// <summary>
        /// Logout an app token.
        ///
        /// Trying to log out other tokens will fail.
        /// </summary>
        public async Task<bool> LogoutAsync(
            AuthenticationLogoutOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _authenticationApi.V1AuthenticationLogoutWithHttpInfoAsync(
                    idempotencyKey: options?.IdempotencyKey,
                    cancellationToken: cancellationToken
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(LogoutAsync)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }

        /// <summary>
        /// Logout an app token.
        ///
        /// Trying to log out other tokens will fail.
        /// </summary>
        public bool Logout(AuthenticationLogoutOptions? options = null)
        {
            try
            {
                var response = _authenticationApi.V1AuthenticationLogoutWithHttpInfo(
                    idempotencyKey: options?.IdempotencyKey
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Logout)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }
    }
}
