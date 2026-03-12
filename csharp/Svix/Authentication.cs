// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
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

    public class AuthenticationStreamPortalAccessOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class AuthenticationRotateStreamPollerTokenOptions : SvixOptionsBase
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
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<AppPortalAccessOut>(
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
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(AppPortalAccessAsync)} failed");

                throw;
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
            appPortalAccessIn =
                appPortalAccessIn ?? throw new ArgumentNullException(nameof(appPortalAccessIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<AppPortalAccessOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/auth/app-portal-access/{app_id}",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: appPortalAccessIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(AppPortalAccess)} failed");

                throw;
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
            applicationTokenExpireIn =
                applicationTokenExpireIn
                ?? throw new ArgumentNullException(nameof(applicationTokenExpireIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
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
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ExpireAllAsync)} failed");

                throw;
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
            applicationTokenExpireIn =
                applicationTokenExpireIn
                ?? throw new ArgumentNullException(nameof(applicationTokenExpireIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Post,
                    path: "/api/v1/auth/app/{app_id}/expire-all",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: applicationTokenExpireIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ExpireAll)} failed");

                throw;
            }
        }

        [Obsolete("Please use `AppPortalAccessAsync` instead")]
        public async Task<DashboardAccessOut> DashboardAccessAsync(
            string appId,
            AuthenticationDashboardAccessOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<DashboardAccessOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/auth/dashboard-access/{app_id}",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(DashboardAccessAsync)} failed");

                throw;
            }
        }

        [Obsolete("Please use `AppPortalAccess` instead")]
        public DashboardAccessOut DashboardAccess(
            string appId,
            AuthenticationDashboardAccessOptions? options = null
        )
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<DashboardAccessOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/auth/dashboard-access/{app_id}",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(DashboardAccess)} failed");

                throw;
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
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Post,
                    path: "/api/v1/auth/logout",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(LogoutAsync)} failed");

                throw;
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
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Post,
                    path: "/api/v1/auth/logout",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Logout)} failed");

                throw;
            }
        }

        /// <summary>
        /// Use this function to get magic links (and authentication codes) for connecting your users to the Stream Consumer Portal.
        /// </summary>
        public async Task<AppPortalAccessOut> StreamPortalAccessAsync(
            string streamId,
            StreamPortalAccessIn streamPortalAccessIn,
            AuthenticationStreamPortalAccessOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            streamPortalAccessIn =
                streamPortalAccessIn
                ?? throw new ArgumentNullException(nameof(streamPortalAccessIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<AppPortalAccessOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/auth/stream-portal-access/{stream_id}",
                    pathParams: new Dictionary<string, string> { { "stream_id", streamId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: streamPortalAccessIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(StreamPortalAccessAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Use this function to get magic links (and authentication codes) for connecting your users to the Stream Consumer Portal.
        /// </summary>
        public AppPortalAccessOut StreamPortalAccess(
            string streamId,
            StreamPortalAccessIn streamPortalAccessIn,
            AuthenticationStreamPortalAccessOptions? options = null
        )
        {
            streamPortalAccessIn =
                streamPortalAccessIn
                ?? throw new ArgumentNullException(nameof(streamPortalAccessIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<AppPortalAccessOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/auth/stream-portal-access/{stream_id}",
                    pathParams: new Dictionary<string, string> { { "stream_id", streamId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: streamPortalAccessIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(StreamPortalAccess)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get the current auth token for the stream poller.
        /// </summary>
        public async Task<ApiTokenOut> GetStreamPollerTokenAsync(
            string streamId,
            string sinkId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<ApiTokenOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/auth/stream/{stream_id}/sink/{sink_id}/poller/token",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(GetStreamPollerTokenAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get the current auth token for the stream poller.
        /// </summary>
        public ApiTokenOut GetStreamPollerToken(string streamId, string sinkId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ApiTokenOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/auth/stream/{stream_id}/sink/{sink_id}/poller/token",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    }
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(GetStreamPollerToken)} failed");

                throw;
            }
        }

        /// <summary>
        /// Create a new auth token for the stream poller API.
        /// </summary>
        public async Task<ApiTokenOut> RotateStreamPollerTokenAsync(
            string streamId,
            string sinkId,
            RotatePollerTokenIn rotatePollerTokenIn,
            AuthenticationRotateStreamPollerTokenOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            rotatePollerTokenIn =
                rotatePollerTokenIn ?? throw new ArgumentNullException(nameof(rotatePollerTokenIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<ApiTokenOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/auth/stream/{stream_id}/sink/{sink_id}/poller/token/rotate",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: rotatePollerTokenIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(RotateStreamPollerTokenAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Create a new auth token for the stream poller API.
        /// </summary>
        public ApiTokenOut RotateStreamPollerToken(
            string streamId,
            string sinkId,
            RotatePollerTokenIn rotatePollerTokenIn,
            AuthenticationRotateStreamPollerTokenOptions? options = null
        )
        {
            rotatePollerTokenIn =
                rotatePollerTokenIn ?? throw new ArgumentNullException(nameof(rotatePollerTokenIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ApiTokenOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/auth/stream/{stream_id}/sink/{sink_id}/poller/token/rotate",
                    pathParams: new Dictionary<string, string>
                    {
                        { "stream_id", streamId },
                        { "sink_id", sinkId },
                    },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: rotatePollerTokenIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(RotateStreamPollerToken)} failed");

                throw;
            }
        }
    }
}
