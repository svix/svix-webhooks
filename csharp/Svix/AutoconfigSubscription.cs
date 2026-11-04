// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class AutoconfigSubscriptionCreateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class AutoconfigSubscriptionRotateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class AutoconfigSubscription(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Create an AutoConfig subscription.
        /// </summary>
        public async Task<AutoConfigOut> CreateAsync(
            string appId,
            CreateAutoConfigSubscriptionIn createAutoConfigSubscriptionIn,
            AutoconfigSubscriptionCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            if (options == null)
            {
                options = new AutoconfigSubscriptionCreateOptions();
            }
            createAutoConfigSubscriptionIn =
                createAutoConfigSubscriptionIn
                ?? throw new ArgumentNullException(nameof(createAutoConfigSubscriptionIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<AutoConfigOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/autoconfig",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options.QueryParams(),
                    headerParams: options.HeaderParams(),
                    content: createAutoConfigSubscriptionIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Create an AutoConfig subscription.
        /// </summary>
        public AutoConfigOut Create(
            string appId,
            CreateAutoConfigSubscriptionIn createAutoConfigSubscriptionIn,
            AutoconfigSubscriptionCreateOptions? options = null
        )
        {
            if (options == null)
            {
                options = new AutoconfigSubscriptionCreateOptions();
            }
            createAutoConfigSubscriptionIn =
                createAutoConfigSubscriptionIn
                ?? throw new ArgumentNullException(nameof(createAutoConfigSubscriptionIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<AutoConfigOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/autoconfig",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options.QueryParams(),
                    headerParams: options.HeaderParams(),
                    content: createAutoConfigSubscriptionIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Create)} failed");

                throw;
            }
        }

        /// <summary>
        /// Delete an AutoConfig subscription. This also invalidates its auth token.
        /// </summary>
        public async Task<bool> DeleteAsync(
            string appId,
            string autoconfigId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "autoconfig_id", autoconfigId },
                    },
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(DeleteAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Delete an AutoConfig subscription. This also invalidates its auth token.
        /// </summary>
        public bool Delete(string appId, string autoconfigId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "autoconfig_id", autoconfigId },
                    }
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Delete)} failed");

                throw;
            }
        }

        /// <summary>
        /// Rotate the auth token and signing secret for an AutoConfig subscription.
        /// </summary>
        public async Task<AutoConfigOut> RotateAsync(
            string appId,
            string autoconfigId,
            RotateSubscriptionIn2 rotateSubscriptionIn2,
            AutoconfigSubscriptionRotateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            if (options == null)
            {
                options = new AutoconfigSubscriptionRotateOptions();
            }
            rotateSubscriptionIn2 =
                rotateSubscriptionIn2
                ?? throw new ArgumentNullException(nameof(rotateSubscriptionIn2));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<AutoConfigOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/rotate",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "autoconfig_id", autoconfigId },
                    },
                    queryParams: options.QueryParams(),
                    headerParams: options.HeaderParams(),
                    content: rotateSubscriptionIn2,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(RotateAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Rotate the auth token and signing secret for an AutoConfig subscription.
        /// </summary>
        public AutoConfigOut Rotate(
            string appId,
            string autoconfigId,
            RotateSubscriptionIn2 rotateSubscriptionIn2,
            AutoconfigSubscriptionRotateOptions? options = null
        )
        {
            if (options == null)
            {
                options = new AutoconfigSubscriptionRotateOptions();
            }
            rotateSubscriptionIn2 =
                rotateSubscriptionIn2
                ?? throw new ArgumentNullException(nameof(rotateSubscriptionIn2));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<AutoConfigOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/rotate",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "autoconfig_id", autoconfigId },
                    },
                    queryParams: options.QueryParams(),
                    headerParams: options.HeaderParams(),
                    content: rotateSubscriptionIn2
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Rotate)} failed");

                throw;
            }
        }
    }
}
