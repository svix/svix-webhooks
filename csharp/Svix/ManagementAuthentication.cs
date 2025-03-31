// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class ManagementAuthenticationListApiTokensOptions : SvixOptionsBase
    {
        public ulong? Limit { get; set; }
        public string? Iterator { get; set; }
        public Ordering? Order { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?>
                {
                    { "limit", Limit },
                    { "iterator", Iterator },
                    { "order", Order },
                }
            );
        }
    }

    public class ManagementAuthenticationCreateApiTokenOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class ManagementAuthenticationExpireApiTokenOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class ManagementAuthentication(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// List all API Tokens.
        /// </summary>
        public async Task<ListResponseApiTokenCensoredOut> ListApiTokensAsync(
            ManagementAuthenticationListApiTokensOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<ListResponseApiTokenCensoredOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/management/authentication/api-token",
                        queryParams: options?.QueryParams(),
                        headerParams: options?.HeaderParams(),
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ListApiTokensAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// List all API Tokens.
        /// </summary>
        public ListResponseApiTokenCensoredOut ListApiTokens(
            ManagementAuthenticationListApiTokensOptions? options = null
        )
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ListResponseApiTokenCensoredOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/management/authentication/api-token",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ListApiTokens)} failed");

                throw;
            }
        }

        /// <summary>
        /// Create a new API Token.
        /// </summary>
        public async Task<ApiTokenOut> CreateApiTokenAsync(
            ApiTokenIn apiTokenIn,
            ManagementAuthenticationCreateApiTokenOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            apiTokenIn = apiTokenIn ?? throw new ArgumentNullException(nameof(apiTokenIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<ApiTokenOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/management/authentication/api-token",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: apiTokenIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(CreateApiTokenAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Create a new API Token.
        /// </summary>
        public ApiTokenOut CreateApiToken(
            ApiTokenIn apiTokenIn,
            ManagementAuthenticationCreateApiTokenOptions? options = null
        )
        {
            apiTokenIn = apiTokenIn ?? throw new ArgumentNullException(nameof(apiTokenIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ApiTokenOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/management/authentication/api-token",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: apiTokenIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(CreateApiToken)} failed");

                throw;
            }
        }

        /// <summary>
        /// Expire the selected API Token.
        /// </summary>
        public async Task<bool> ExpireApiTokenAsync(
            string keyId,
            ApiTokenExpireIn apiTokenExpireIn,
            ManagementAuthenticationExpireApiTokenOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            apiTokenExpireIn =
                apiTokenExpireIn ?? throw new ArgumentNullException(nameof(apiTokenExpireIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Post,
                    path: "/api/v1/management/authentication/api-token/{key_id}/expire",
                    pathParams: new Dictionary<string, string> { { "key_id", keyId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: apiTokenExpireIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ExpireApiTokenAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Expire the selected API Token.
        /// </summary>
        public bool ExpireApiToken(
            string keyId,
            ApiTokenExpireIn apiTokenExpireIn,
            ManagementAuthenticationExpireApiTokenOptions? options = null
        )
        {
            apiTokenExpireIn =
                apiTokenExpireIn ?? throw new ArgumentNullException(nameof(apiTokenExpireIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Post,
                    path: "/api/v1/management/authentication/api-token/{key_id}/expire",
                    pathParams: new Dictionary<string, string> { { "key_id", keyId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: apiTokenExpireIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ExpireApiToken)} failed");

                throw;
            }
        }
    }
}
