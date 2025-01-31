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
    public partial class EndpointListOptions
    {
        public int? Limit { get; set; }
        public string? Iterator { get; set; }
        public Ordering? Order { get; set; }
    }

    public partial class EndpointCreateOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public partial class EndpointRecoverOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public partial class EndpointReplayMissingOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public partial class EndpointRotateSecretOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public partial class EndpointSendExampleOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public partial class EndpointGetStatsOptions
    {
        public DateTime? Since { get; set; }
        public DateTime? Until { get; set; }
    }

    public sealed class Endpoint : SvixResourceBase
    {
        private readonly EndpointApi _endpointApi;

        public Endpoint(ISvixClient svixClient, EndpointApi endpointApi)
            : base(svixClient)
        {
            _endpointApi = endpointApi ?? throw new ArgumentNullException(nameof(endpointApi));
        }

        /// <summary>
        /// List the application's endpoints.
        /// </summary>
        public async Task<ListResponseEndpointOut> ListAsync(
            string appId,
            EndpointListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _endpointApi.V1EndpointListWithHttpInfoAsync(
                    appId: appId,
                    limit: options?.Limit,
                    iterator: options?.Iterator,
                    order: options?.Order,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;
                return new ListResponseEndpointOut();
            }
        }

        /// <summary>
        /// List the application's endpoints.
        /// </summary>
        public ListResponseEndpointOut List(string appId, EndpointListOptions? options = null)
        {
            try
            {
                var response = _endpointApi.V1EndpointListWithHttpInfo(
                    appId: appId,
                    limit: options?.Limit,
                    iterator: options?.Iterator,
                    order: options?.Order
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;
                return new ListResponseEndpointOut();
            }
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
            try
            {
                endpointIn = endpointIn ?? throw new ArgumentNullException(nameof(endpointIn));
                var response = await _endpointApi.V1EndpointCreateWithHttpInfoAsync(
                    appId: appId,
                    endpointIn: endpointIn,
                    idempotencyKey: options?.IdempotencyKey,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                if (Throw)
                    throw;
                return new EndpointOut();
            }
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
            try
            {
                endpointIn = endpointIn ?? throw new ArgumentNullException(nameof(endpointIn));
                var response = _endpointApi.V1EndpointCreateWithHttpInfo(
                    appId: appId,
                    endpointIn: endpointIn,
                    idempotencyKey: options?.IdempotencyKey
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Create)} failed");

                if (Throw)
                    throw;
                return new EndpointOut();
            }
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
            try
            {
                var response = await _endpointApi.V1EndpointGetWithHttpInfoAsync(
                    appId: appId,
                    endpointId: endpointId,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                if (Throw)
                    throw;
                return new EndpointOut();
            }
        }

        /// <summary>
        /// Get an endpoint.
        /// </summary>
        public EndpointOut Get(string appId, string endpointId)
        {
            try
            {
                var response = _endpointApi.V1EndpointGetWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Get)} failed");

                if (Throw)
                    throw;
                return new EndpointOut();
            }
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
            try
            {
                endpointUpdate =
                    endpointUpdate ?? throw new ArgumentNullException(nameof(endpointUpdate));
                var response = await _endpointApi.V1EndpointUpdateWithHttpInfoAsync(
                    appId: appId,
                    endpointId: endpointId,
                    endpointUpdate: endpointUpdate,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(UpdateAsync)} failed");

                if (Throw)
                    throw;
                return new EndpointOut();
            }
        }

        /// <summary>
        /// Update an endpoint.
        /// </summary>
        public EndpointOut Update(string appId, string endpointId, EndpointUpdate endpointUpdate)
        {
            try
            {
                endpointUpdate =
                    endpointUpdate ?? throw new ArgumentNullException(nameof(endpointUpdate));
                var response = _endpointApi.V1EndpointUpdateWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId,
                    endpointUpdate: endpointUpdate
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Update)} failed");

                if (Throw)
                    throw;
                return new EndpointOut();
            }
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
            try
            {
                var response = await _endpointApi.V1EndpointDeleteWithHttpInfoAsync(
                    appId: appId,
                    endpointId: endpointId,
                    cancellationToken: cancellationToken
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(DeleteAsync)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }

        /// <summary>
        /// Delete an endpoint.
        /// </summary>
        public bool Delete(string appId, string endpointId)
        {
            try
            {
                var response = _endpointApi.V1EndpointDeleteWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Delete)} failed");

                if (Throw)
                    throw;
                return false;
            }
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
            try
            {
                endpointPatch =
                    endpointPatch ?? throw new ArgumentNullException(nameof(endpointPatch));
                var response = await _endpointApi.V1EndpointPatchWithHttpInfoAsync(
                    appId: appId,
                    endpointId: endpointId,
                    endpointPatch: endpointPatch,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(PatchAsync)} failed");

                if (Throw)
                    throw;
                return new EndpointOut();
            }
        }

        /// <summary>
        /// Partially update an endpoint.
        /// </summary>
        public EndpointOut Patch(string appId, string endpointId, EndpointPatch endpointPatch)
        {
            try
            {
                endpointPatch =
                    endpointPatch ?? throw new ArgumentNullException(nameof(endpointPatch));
                var response = _endpointApi.V1EndpointPatchWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId,
                    endpointPatch: endpointPatch
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Patch)} failed");

                if (Throw)
                    throw;
                return new EndpointOut();
            }
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
            try
            {
                var response = await _endpointApi.V1EndpointGetHeadersWithHttpInfoAsync(
                    appId: appId,
                    endpointId: endpointId,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetHeadersAsync)} failed");

                if (Throw)
                    throw;
                return new EndpointHeadersOut();
            }
        }

        /// <summary>
        /// Get the additional headers to be sent with the webhook.
        /// </summary>
        public EndpointHeadersOut GetHeaders(string appId, string endpointId)
        {
            try
            {
                var response = _endpointApi.V1EndpointGetHeadersWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetHeaders)} failed");

                if (Throw)
                    throw;
                return new EndpointHeadersOut();
            }
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
            try
            {
                endpointHeadersIn =
                    endpointHeadersIn ?? throw new ArgumentNullException(nameof(endpointHeadersIn));
                var response = await _endpointApi.V1EndpointUpdateHeadersWithHttpInfoAsync(
                    appId: appId,
                    endpointId: endpointId,
                    endpointHeadersIn: endpointHeadersIn,
                    cancellationToken: cancellationToken
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(UpdateHeadersAsync)} failed");

                if (Throw)
                    throw;
                return false;
            }
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
            try
            {
                endpointHeadersIn =
                    endpointHeadersIn ?? throw new ArgumentNullException(nameof(endpointHeadersIn));
                var response = _endpointApi.V1EndpointUpdateHeadersWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId,
                    endpointHeadersIn: endpointHeadersIn
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(UpdateHeaders)} failed");

                if (Throw)
                    throw;
                return false;
            }
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
            try
            {
                endpointHeadersPatchIn =
                    endpointHeadersPatchIn
                    ?? throw new ArgumentNullException(nameof(endpointHeadersPatchIn));
                var response = await _endpointApi.V1EndpointPatchHeadersWithHttpInfoAsync(
                    appId: appId,
                    endpointId: endpointId,
                    endpointHeadersPatchIn: endpointHeadersPatchIn,
                    cancellationToken: cancellationToken
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(PatchHeadersAsync)} failed");

                if (Throw)
                    throw;
                return false;
            }
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
            try
            {
                endpointHeadersPatchIn =
                    endpointHeadersPatchIn
                    ?? throw new ArgumentNullException(nameof(endpointHeadersPatchIn));
                var response = _endpointApi.V1EndpointPatchHeadersWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId,
                    endpointHeadersPatchIn: endpointHeadersPatchIn
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(PatchHeaders)} failed");

                if (Throw)
                    throw;
                return false;
            }
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
            try
            {
                recoverIn = recoverIn ?? throw new ArgumentNullException(nameof(recoverIn));
                var response = await _endpointApi.V1EndpointRecoverWithHttpInfoAsync(
                    appId: appId,
                    endpointId: endpointId,
                    recoverIn: recoverIn,
                    idempotencyKey: options?.IdempotencyKey,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(RecoverAsync)} failed");

                if (Throw)
                    throw;
                return new RecoverOut();
            }
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
            try
            {
                recoverIn = recoverIn ?? throw new ArgumentNullException(nameof(recoverIn));
                var response = _endpointApi.V1EndpointRecoverWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId,
                    recoverIn: recoverIn,
                    idempotencyKey: options?.IdempotencyKey
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Recover)} failed");

                if (Throw)
                    throw;
                return new RecoverOut();
            }
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
            try
            {
                replayIn = replayIn ?? throw new ArgumentNullException(nameof(replayIn));
                var response = await _endpointApi.V1EndpointReplayMissingWithHttpInfoAsync(
                    appId: appId,
                    endpointId: endpointId,
                    replayIn: replayIn,
                    idempotencyKey: options?.IdempotencyKey,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ReplayMissingAsync)} failed");

                if (Throw)
                    throw;
                return new ReplayOut();
            }
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
            try
            {
                replayIn = replayIn ?? throw new ArgumentNullException(nameof(replayIn));
                var response = _endpointApi.V1EndpointReplayMissingWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId,
                    replayIn: replayIn,
                    idempotencyKey: options?.IdempotencyKey
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ReplayMissing)} failed");

                if (Throw)
                    throw;
                return new ReplayOut();
            }
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
            try
            {
                var response = await _endpointApi.V1EndpointGetSecretWithHttpInfoAsync(
                    appId: appId,
                    endpointId: endpointId,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetSecretAsync)} failed");

                if (Throw)
                    throw;
                return new EndpointSecretOut();
            }
        }

        /// <summary>
        /// Get the endpoint's signing secret.
        ///
        /// This is used to verify the authenticity of the webhook.
        /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
        /// </summary>
        public EndpointSecretOut GetSecret(string appId, string endpointId)
        {
            try
            {
                var response = _endpointApi.V1EndpointGetSecretWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetSecret)} failed");

                if (Throw)
                    throw;
                return new EndpointSecretOut();
            }
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
            try
            {
                endpointSecretRotateIn =
                    endpointSecretRotateIn
                    ?? throw new ArgumentNullException(nameof(endpointSecretRotateIn));
                var response = await _endpointApi.V1EndpointRotateSecretWithHttpInfoAsync(
                    appId: appId,
                    endpointId: endpointId,
                    endpointSecretRotateIn: endpointSecretRotateIn,
                    idempotencyKey: options?.IdempotencyKey,
                    cancellationToken: cancellationToken
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(RotateSecretAsync)} failed");

                if (Throw)
                    throw;
                return false;
            }
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
            try
            {
                endpointSecretRotateIn =
                    endpointSecretRotateIn
                    ?? throw new ArgumentNullException(nameof(endpointSecretRotateIn));
                var response = _endpointApi.V1EndpointRotateSecretWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId,
                    endpointSecretRotateIn: endpointSecretRotateIn,
                    idempotencyKey: options?.IdempotencyKey
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(RotateSecret)} failed");

                if (Throw)
                    throw;
                return false;
            }
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
            try
            {
                eventExampleIn =
                    eventExampleIn ?? throw new ArgumentNullException(nameof(eventExampleIn));
                var response = await _endpointApi.V1EndpointSendExampleWithHttpInfoAsync(
                    appId: appId,
                    endpointId: endpointId,
                    eventExampleIn: eventExampleIn,
                    idempotencyKey: options?.IdempotencyKey,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(SendExampleAsync)} failed");

                if (Throw)
                    throw;
                return new MessageOut();
            }
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
            try
            {
                eventExampleIn =
                    eventExampleIn ?? throw new ArgumentNullException(nameof(eventExampleIn));
                var response = _endpointApi.V1EndpointSendExampleWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId,
                    eventExampleIn: eventExampleIn,
                    idempotencyKey: options?.IdempotencyKey
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(SendExample)} failed");

                if (Throw)
                    throw;
                return new MessageOut();
            }
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
            try
            {
                var response = await _endpointApi.V1EndpointGetStatsWithHttpInfoAsync(
                    appId: appId,
                    endpointId: endpointId,
                    since: options?.Since,
                    until: options?.Until,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetStatsAsync)} failed");

                if (Throw)
                    throw;
                return new EndpointStats();
            }
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
            try
            {
                var response = _endpointApi.V1EndpointGetStatsWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId,
                    since: options?.Since,
                    until: options?.Until
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetStats)} failed");

                if (Throw)
                    throw;
                return new EndpointStats();
            }
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
            try
            {
                var response = await _endpointApi.V1EndpointTransformationGetWithHttpInfoAsync(
                    appId: appId,
                    endpointId: endpointId,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(TransformationGetAsync)} failed");

                if (Throw)
                    throw;
                return new EndpointTransformationOut();
            }
        }

        /// <summary>
        /// Get the transformation code associated with this endpoint.
        /// </summary>
        public EndpointTransformationOut TransformationGet(string appId, string endpointId)
        {
            try
            {
                var response = _endpointApi.V1EndpointTransformationGetWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(TransformationGet)} failed");

                if (Throw)
                    throw;
                return new EndpointTransformationOut();
            }
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
            try
            {
                endpointTransformationIn =
                    endpointTransformationIn
                    ?? throw new ArgumentNullException(nameof(endpointTransformationIn));
                var response =
                    await _endpointApi.V1EndpointTransformationPartialUpdateWithHttpInfoAsync(
                        appId: appId,
                        endpointId: endpointId,
                        endpointTransformationIn: endpointTransformationIn,
                        cancellationToken: cancellationToken
                    );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(TransformationPartialUpdateAsync)} failed");

                if (Throw)
                    throw;
                return false;
            }
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
            try
            {
                endpointTransformationIn =
                    endpointTransformationIn
                    ?? throw new ArgumentNullException(nameof(endpointTransformationIn));
                var response = _endpointApi.V1EndpointTransformationPartialUpdateWithHttpInfo(
                    appId: appId,
                    endpointId: endpointId,
                    endpointTransformationIn: endpointTransformationIn
                );
                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(TransformationPartialUpdate)} failed");

                if (Throw)
                    throw;
                return false;
            }
        }
    }
}
