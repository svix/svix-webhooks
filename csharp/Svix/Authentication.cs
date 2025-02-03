// this file is @generated
#nullable enable
using Svix.Models;

namespace Svix
{
    public class AuthenticationAppPortalAccessOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class AuthenticationExpireAllOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class AuthenticationDashboardAccessOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class AuthenticationLogoutOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class Authentication(SvixClient client)
    {
        readonly SvixClient _client = client;

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
            appPortalAccessIn =
                appPortalAccessIn ?? throw new ArgumentNullException(nameof(appPortalAccessIn));

            var response = await this._client.SvixHttpClient.SendRequestAsync<AppPortalAccessOut>(
                method: HttpMethod.Post,
                path: "/api/v1/auth/app-portal-access/{app_id}",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: appPortalAccessIn,
                cancellationToken: cancellationToken
            );
            return response.Data;
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
            appPortalAccessIn =
                appPortalAccessIn ?? throw new ArgumentNullException(nameof(appPortalAccessIn));

            var response = this._client.SvixHttpClient.SendRequest<AppPortalAccessOut>(
                method: HttpMethod.Post,
                path: "/api/v1/auth/app-portal-access/{app_id}",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: appPortalAccessIn
            );
            return response.Data;
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
            applicationTokenExpireIn =
                applicationTokenExpireIn
                ?? throw new ArgumentNullException(nameof(applicationTokenExpireIn));

            var response = await this._client.SvixHttpClient.SendRequestAsync<bool>(
                method: HttpMethod.Post,
                path: "/api/v1/auth/app/{app_id}/expire-all",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: applicationTokenExpireIn,
                cancellationToken: cancellationToken
            );
            return response.Data;
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
            applicationTokenExpireIn =
                applicationTokenExpireIn
                ?? throw new ArgumentNullException(nameof(applicationTokenExpireIn));

            var response = this._client.SvixHttpClient.SendRequest<bool>(
                method: HttpMethod.Post,
                path: "/api/v1/auth/app/{app_id}/expire-all",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: applicationTokenExpireIn
            );
            return response.Data;
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
            var response = await this._client.SvixHttpClient.SendRequestAsync<DashboardAccessOut>(
                method: HttpMethod.Post,
                path: "/api/v1/auth/dashboard-access/{app_id}",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                cancellationToken: cancellationToken
            );
            return response.Data;
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
            var response = this._client.SvixHttpClient.SendRequest<DashboardAccessOut>(
                method: HttpMethod.Post,
                path: "/api/v1/auth/dashboard-access/{app_id}",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams()
            );
            return response.Data;
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
            var response = await this._client.SvixHttpClient.SendRequestAsync<bool>(
                method: HttpMethod.Post,
                path: "/api/v1/auth/logout",
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Logout an app token.
        ///
        /// Trying to log out other tokens will fail.
        /// </summary>
        public bool Logout(AuthenticationLogoutOptions? options = null)
        {
            var response = this._client.SvixHttpClient.SendRequest<bool>(
                method: HttpMethod.Post,
                path: "/api/v1/auth/logout",
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams()
            );
            return response.Data;
        }
    }
}
