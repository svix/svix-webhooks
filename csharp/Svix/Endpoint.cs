// this file is @generated
#nullable enable
using Svix.Models;

namespace Svix
{
    public class EndpointListOptions : SvixOptionsBase
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

    public class EndpointCreateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class EndpointRecoverOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class EndpointReplayMissingOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class EndpointRotateSecretOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class EndpointSendExampleOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class EndpointGetStatsOptions : SvixOptionsBase
    {
        public DateTime? Since { get; set; }
        public DateTime? Until { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "since", Since }, { "until", Until } }
            );
        }
    }

    public class Endpoint(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// List the application's endpoints.
        /// </summary>
        public async Task<ListResponseEndpointOut> ListAsync(
            string appId,
            EndpointListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            var response =
                await this._client.SvixHttpClient.SendRequestAsync<ListResponseEndpointOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/endpoint",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
            return response.Data;
        }

        /// <summary>
        /// List the application's endpoints.
        /// </summary>
        public ListResponseEndpointOut List(string appId, EndpointListOptions? options = null)
        {
            var response = this._client.SvixHttpClient.SendRequest<ListResponseEndpointOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/endpoint",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams()
            );
            return response.Data;
        }

        /// <summary>
        /// Create a new endpoint for the application.
        ///
        /// When `secret` is `null` the secret is automatically generated (recommended).
        /// </summary>
        public async Task<EndpointOut> CreateAsync(
            string appId,
            EndpointIn endpointIn,
            EndpointCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            endpointIn = endpointIn ?? throw new ArgumentNullException(nameof(endpointIn));

            var response = await this._client.SvixHttpClient.SendRequestAsync<EndpointOut>(
                method: HttpMethod.Post,
                path: "/api/v1/app/{app_id}/endpoint",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: endpointIn,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Create a new endpoint for the application.
        ///
        /// When `secret` is `null` the secret is automatically generated (recommended).
        /// </summary>
        public EndpointOut Create(
            string appId,
            EndpointIn endpointIn,
            EndpointCreateOptions? options = null
        )
        {
            endpointIn = endpointIn ?? throw new ArgumentNullException(nameof(endpointIn));

            var response = this._client.SvixHttpClient.SendRequest<EndpointOut>(
                method: HttpMethod.Post,
                path: "/api/v1/app/{app_id}/endpoint",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: endpointIn
            );
            return response.Data;
        }

        /// <summary>
        /// Get an endpoint.
        /// </summary>
        public async Task<EndpointOut> GetAsync(
            string appId,
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            var response = await this._client.SvixHttpClient.SendRequestAsync<EndpointOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Get an endpoint.
        /// </summary>
        public EndpointOut Get(string appId, string endpointId)
        {
            var response = this._client.SvixHttpClient.SendRequest<EndpointOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                }
            );
            return response.Data;
        }

        /// <summary>
        /// Update an endpoint.
        /// </summary>
        public async Task<EndpointOut> UpdateAsync(
            string appId,
            string endpointId,
            EndpointUpdate endpointUpdate,
            CancellationToken cancellationToken = default
        )
        {
            endpointUpdate =
                endpointUpdate ?? throw new ArgumentNullException(nameof(endpointUpdate));

            var response = await this._client.SvixHttpClient.SendRequestAsync<EndpointOut>(
                method: HttpMethod.Put,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                content: endpointUpdate,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Update an endpoint.
        /// </summary>
        public EndpointOut Update(string appId, string endpointId, EndpointUpdate endpointUpdate)
        {
            endpointUpdate =
                endpointUpdate ?? throw new ArgumentNullException(nameof(endpointUpdate));

            var response = this._client.SvixHttpClient.SendRequest<EndpointOut>(
                method: HttpMethod.Put,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                content: endpointUpdate
            );
            return response.Data;
        }

        /// <summary>
        /// Delete an endpoint.
        /// </summary>
        public async Task<bool> DeleteAsync(
            string appId,
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            var response = await this._client.SvixHttpClient.SendRequestAsync<bool>(
                method: HttpMethod.Delete,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Delete an endpoint.
        /// </summary>
        public bool Delete(string appId, string endpointId)
        {
            var response = this._client.SvixHttpClient.SendRequest<bool>(
                method: HttpMethod.Delete,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                }
            );
            return response.Data;
        }

        /// <summary>
        /// Partially update an endpoint.
        /// </summary>
        public async Task<EndpointOut> PatchAsync(
            string appId,
            string endpointId,
            EndpointPatch endpointPatch,
            CancellationToken cancellationToken = default
        )
        {
            endpointPatch = endpointPatch ?? throw new ArgumentNullException(nameof(endpointPatch));

            var response = await this._client.SvixHttpClient.SendRequestAsync<EndpointOut>(
                method: HttpMethod.Patch,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                content: endpointPatch,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Partially update an endpoint.
        /// </summary>
        public EndpointOut Patch(string appId, string endpointId, EndpointPatch endpointPatch)
        {
            endpointPatch = endpointPatch ?? throw new ArgumentNullException(nameof(endpointPatch));

            var response = this._client.SvixHttpClient.SendRequest<EndpointOut>(
                method: HttpMethod.Patch,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                content: endpointPatch
            );
            return response.Data;
        }

        /// <summary>
        /// Get the additional headers to be sent with the webhook.
        /// </summary>
        public async Task<EndpointHeadersOut> GetHeadersAsync(
            string appId,
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            var response = await this._client.SvixHttpClient.SendRequestAsync<EndpointHeadersOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Get the additional headers to be sent with the webhook.
        /// </summary>
        public EndpointHeadersOut GetHeaders(string appId, string endpointId)
        {
            var response = this._client.SvixHttpClient.SendRequest<EndpointHeadersOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                }
            );
            return response.Data;
        }

        /// <summary>
        /// Set the additional headers to be sent with the webhook.
        /// </summary>
        public async Task<bool> UpdateHeadersAsync(
            string appId,
            string endpointId,
            EndpointHeadersIn endpointHeadersIn,
            CancellationToken cancellationToken = default
        )
        {
            endpointHeadersIn =
                endpointHeadersIn ?? throw new ArgumentNullException(nameof(endpointHeadersIn));

            var response = await this._client.SvixHttpClient.SendRequestAsync<bool>(
                method: HttpMethod.Put,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                content: endpointHeadersIn,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Set the additional headers to be sent with the webhook.
        /// </summary>
        public bool UpdateHeaders(
            string appId,
            string endpointId,
            EndpointHeadersIn endpointHeadersIn
        )
        {
            endpointHeadersIn =
                endpointHeadersIn ?? throw new ArgumentNullException(nameof(endpointHeadersIn));

            var response = this._client.SvixHttpClient.SendRequest<bool>(
                method: HttpMethod.Put,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                content: endpointHeadersIn
            );
            return response.Data;
        }

        /// <summary>
        /// Partially set the additional headers to be sent with the webhook.
        /// </summary>
        public async Task<bool> PatchHeadersAsync(
            string appId,
            string endpointId,
            EndpointHeadersPatchIn endpointHeadersPatchIn,
            CancellationToken cancellationToken = default
        )
        {
            endpointHeadersPatchIn =
                endpointHeadersPatchIn
                ?? throw new ArgumentNullException(nameof(endpointHeadersPatchIn));

            var response = await this._client.SvixHttpClient.SendRequestAsync<bool>(
                method: HttpMethod.Patch,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                content: endpointHeadersPatchIn,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Partially set the additional headers to be sent with the webhook.
        /// </summary>
        public bool PatchHeaders(
            string appId,
            string endpointId,
            EndpointHeadersPatchIn endpointHeadersPatchIn
        )
        {
            endpointHeadersPatchIn =
                endpointHeadersPatchIn
                ?? throw new ArgumentNullException(nameof(endpointHeadersPatchIn));

            var response = this._client.SvixHttpClient.SendRequest<bool>(
                method: HttpMethod.Patch,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                content: endpointHeadersPatchIn
            );
            return response.Data;
        }

        /// <summary>
        /// Resend all failed messages since a given time.
        ///
        /// Messages that were sent successfully, even if failed initially, are not resent.
        /// </summary>
        public async Task<RecoverOut> RecoverAsync(
            string appId,
            string endpointId,
            RecoverIn recoverIn,
            EndpointRecoverOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            recoverIn = recoverIn ?? throw new ArgumentNullException(nameof(recoverIn));

            var response = await this._client.SvixHttpClient.SendRequestAsync<RecoverOut>(
                method: HttpMethod.Post,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/recover",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: recoverIn,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Resend all failed messages since a given time.
        ///
        /// Messages that were sent successfully, even if failed initially, are not resent.
        /// </summary>
        public RecoverOut Recover(
            string appId,
            string endpointId,
            RecoverIn recoverIn,
            EndpointRecoverOptions? options = null
        )
        {
            recoverIn = recoverIn ?? throw new ArgumentNullException(nameof(recoverIn));

            var response = this._client.SvixHttpClient.SendRequest<RecoverOut>(
                method: HttpMethod.Post,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/recover",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: recoverIn
            );
            return response.Data;
        }

        /// <summary>
        /// Replays messages to the endpoint.
        ///
        /// Only messages that were created after `since` will be sent.
        /// Messages that were previously sent to the endpoint are not resent.
        /// </summary>
        public async Task<ReplayOut> ReplayMissingAsync(
            string appId,
            string endpointId,
            ReplayIn replayIn,
            EndpointReplayMissingOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            replayIn = replayIn ?? throw new ArgumentNullException(nameof(replayIn));

            var response = await this._client.SvixHttpClient.SendRequestAsync<ReplayOut>(
                method: HttpMethod.Post,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/replay-missing",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: replayIn,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Replays messages to the endpoint.
        ///
        /// Only messages that were created after `since` will be sent.
        /// Messages that were previously sent to the endpoint are not resent.
        /// </summary>
        public ReplayOut ReplayMissing(
            string appId,
            string endpointId,
            ReplayIn replayIn,
            EndpointReplayMissingOptions? options = null
        )
        {
            replayIn = replayIn ?? throw new ArgumentNullException(nameof(replayIn));

            var response = this._client.SvixHttpClient.SendRequest<ReplayOut>(
                method: HttpMethod.Post,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/replay-missing",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: replayIn
            );
            return response.Data;
        }

        /// <summary>
        /// Get the endpoint's signing secret.
        ///
        /// This is used to verify the authenticity of the webhook.
        /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
        /// </summary>
        public async Task<EndpointSecretOut> GetSecretAsync(
            string appId,
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            var response = await this._client.SvixHttpClient.SendRequestAsync<EndpointSecretOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/secret",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Get the endpoint's signing secret.
        ///
        /// This is used to verify the authenticity of the webhook.
        /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
        /// </summary>
        public EndpointSecretOut GetSecret(string appId, string endpointId)
        {
            var response = this._client.SvixHttpClient.SendRequest<EndpointSecretOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/secret",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                }
            );
            return response.Data;
        }

        /// <summary>
        /// Rotates the endpoint's signing secret.
        ///
        /// The previous secret will remain valid for the next 24 hours.
        /// </summary>
        public async Task<bool> RotateSecretAsync(
            string appId,
            string endpointId,
            EndpointSecretRotateIn endpointSecretRotateIn,
            EndpointRotateSecretOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            endpointSecretRotateIn =
                endpointSecretRotateIn
                ?? throw new ArgumentNullException(nameof(endpointSecretRotateIn));

            var response = await this._client.SvixHttpClient.SendRequestAsync<bool>(
                method: HttpMethod.Post,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/secret/rotate",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: endpointSecretRotateIn,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Rotates the endpoint's signing secret.
        ///
        /// The previous secret will remain valid for the next 24 hours.
        /// </summary>
        public bool RotateSecret(
            string appId,
            string endpointId,
            EndpointSecretRotateIn endpointSecretRotateIn,
            EndpointRotateSecretOptions? options = null
        )
        {
            endpointSecretRotateIn =
                endpointSecretRotateIn
                ?? throw new ArgumentNullException(nameof(endpointSecretRotateIn));

            var response = this._client.SvixHttpClient.SendRequest<bool>(
                method: HttpMethod.Post,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/secret/rotate",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: endpointSecretRotateIn
            );
            return response.Data;
        }

        /// <summary>
        /// Send an example message for an event.
        /// </summary>
        public async Task<MessageOut> SendExampleAsync(
            string appId,
            string endpointId,
            EventExampleIn eventExampleIn,
            EndpointSendExampleOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            eventExampleIn =
                eventExampleIn ?? throw new ArgumentNullException(nameof(eventExampleIn));

            var response = await this._client.SvixHttpClient.SendRequestAsync<MessageOut>(
                method: HttpMethod.Post,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/send-example",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: eventExampleIn,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Send an example message for an event.
        /// </summary>
        public MessageOut SendExample(
            string appId,
            string endpointId,
            EventExampleIn eventExampleIn,
            EndpointSendExampleOptions? options = null
        )
        {
            eventExampleIn =
                eventExampleIn ?? throw new ArgumentNullException(nameof(eventExampleIn));

            var response = this._client.SvixHttpClient.SendRequest<MessageOut>(
                method: HttpMethod.Post,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/send-example",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: eventExampleIn
            );
            return response.Data;
        }

        /// <summary>
        /// Get basic statistics for the endpoint.
        /// </summary>
        public async Task<EndpointStats> GetStatsAsync(
            string appId,
            string endpointId,
            EndpointGetStatsOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            var response = await this._client.SvixHttpClient.SendRequestAsync<EndpointStats>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/stats",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Get basic statistics for the endpoint.
        /// </summary>
        public EndpointStats GetStats(
            string appId,
            string endpointId,
            EndpointGetStatsOptions? options = null
        )
        {
            var response = this._client.SvixHttpClient.SendRequest<EndpointStats>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/stats",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams()
            );
            return response.Data;
        }

        /// <summary>
        /// Get the transformation code associated with this endpoint.
        /// </summary>
        public async Task<EndpointTransformationOut> TransformationGetAsync(
            string appId,
            string endpointId,
            CancellationToken cancellationToken = default
        )
        {
            var response =
                await this._client.SvixHttpClient.SendRequestAsync<EndpointTransformationOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "endpoint_id", endpointId },
                    },
                    cancellationToken: cancellationToken
                );
            return response.Data;
        }

        /// <summary>
        /// Get the transformation code associated with this endpoint.
        /// </summary>
        public EndpointTransformationOut TransformationGet(string appId, string endpointId)
        {
            var response = this._client.SvixHttpClient.SendRequest<EndpointTransformationOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                }
            );
            return response.Data;
        }

        /// <summary>
        /// Set or unset the transformation code associated with this endpoint.
        /// </summary>
        public async Task<bool> TransformationPartialUpdateAsync(
            string appId,
            string endpointId,
            EndpointTransformationIn endpointTransformationIn,
            CancellationToken cancellationToken = default
        )
        {
            endpointTransformationIn =
                endpointTransformationIn
                ?? throw new ArgumentNullException(nameof(endpointTransformationIn));

            var response = await this._client.SvixHttpClient.SendRequestAsync<bool>(
                method: HttpMethod.Patch,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                content: endpointTransformationIn,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Set or unset the transformation code associated with this endpoint.
        /// </summary>
        public bool TransformationPartialUpdate(
            string appId,
            string endpointId,
            EndpointTransformationIn endpointTransformationIn
        )
        {
            endpointTransformationIn =
                endpointTransformationIn
                ?? throw new ArgumentNullException(nameof(endpointTransformationIn));

            var response = this._client.SvixHttpClient.SendRequest<bool>(
                method: HttpMethod.Patch,
                path: "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
                pathParams: new Dictionary<string, string>
                {
                    { "app_id", appId },
                    { "endpoint_id", endpointId },
                },
                content: endpointTransformationIn
            );
            return response.Data;
        }
    }
}
