// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class EndpointAutoconfigCreateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class EndpointAutoconfigRotateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class EndpointAutoconfig(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Create an AutoConfig subscription.
        /// </summary>
        public async Task<AutoConfigOut> CreateAsync(
            string appId,
            EndpointAutoconfigCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            if (options == null)
            {
                options = new EndpointAutoconfigCreateOptions();
            }
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<AutoConfigOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/autoconfig",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options.QueryParams(),
                    headerParams: options.HeaderParams(),
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
        public AutoConfigOut Create(string appId, EndpointAutoconfigCreateOptions? options = null)
        {
            if (options == null)
            {
                options = new EndpointAutoconfigCreateOptions();
            }
            try
            {
                var response = _client.SvixHttpClient.SendRequest<AutoConfigOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/autoconfig",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options.QueryParams(),
                    headerParams: options.HeaderParams()
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
        /// Rotate the auth token and signing secret for an AutoConfig subscription.
        /// </summary>
        public async Task<AutoConfigOut> RotateAsync(
            string appId,
            string autoconfigId,
            RotateSubscriptionIn2 rotateSubscriptionIn2,
            EndpointAutoconfigRotateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            if (options == null)
            {
                options = new EndpointAutoconfigRotateOptions();
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
            EndpointAutoconfigRotateOptions? options = null
        )
        {
            if (options == null)
            {
                options = new EndpointAutoconfigRotateOptions();
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
