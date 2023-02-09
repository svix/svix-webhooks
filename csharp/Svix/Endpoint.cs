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
    public sealed class Endpoint : SvixResourceBase, IEndpoint
    {
        private readonly IEndpointApi _endpointApi;

        public Endpoint(ISvixClient svixClient, IEndpointApi endpoingApi)
            : base(svixClient)
        {
            _endpointApi = endpoingApi ?? throw new ArgumentNullException(nameof(_endpointApi));
        }

        public EndpointOut Create(string appId, EndpointIn endpoint, string idempotencyKey = default)
        {
            try
            {
                var lEndpoint = _endpointApi.CreateEndpointApiV1AppAppIdEndpointPost(
                    appId,
                    endpoint,
                    idempotencyKey);

                return lEndpoint;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Create)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<EndpointOut> CreateAsync(string appId, EndpointIn endpoint, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lEndpoint = await _endpointApi.CreateEndpointApiV1AppAppIdEndpointPostAsync(
                    appId,
                    endpoint,
                    idempotencyKey,
                    cancellationToken);

                return lEndpoint;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public bool Delete(string appId, string endpointId, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _endpointApi.DeleteEndpointApiV1AppAppIdEndpointEndpointIdDeleteWithHttpInfo(
                    endpointId,
                    appId,
                    idempotencyKey);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Delete)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public async Task<bool> DeleteAsync(string appId, string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _endpointApi.DeleteEndpointApiV1AppAppIdEndpointEndpointIdDeleteWithHttpInfoAsync(
                    endpointId,
                    appId,
                    idempotencyKey,
                    cancellationToken);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(DeleteAsync)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public EndpointOut Get(string appId, string endpointId, string idempotencyKey = default)
        {
            try
            {
                var lEndpoint = _endpointApi.GetEndpointApiV1AppAppIdEndpointEndpointIdGet(
                    endpointId,
                    appId,
                    idempotencyKey);

                return lEndpoint;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Get)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<EndpointOut> GetAsync(string appId, string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lEndpoint = await _endpointApi.GetEndpointApiV1AppAppIdEndpointEndpointIdGetAsync(
                    endpointId,
                    appId,
                    idempotencyKey,
                    cancellationToken);

                return lEndpoint;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public EndpointHeadersOut GetHeaders(string appId, string endpointId, string idempotencyKey = default)
        {
            try
            {
                var lHeaders = _endpointApi.GetEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersGet(
                    endpointId,
                    appId,
                    idempotencyKey);

                return lHeaders;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetHeaders)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<EndpointHeadersOut> GetHeadersAsync(string appId, string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lHeaders = await _endpointApi.GetEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersGetAsync(
                    endpointId,
                    appId,
                    idempotencyKey,
                    cancellationToken);

                return lHeaders;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetHeadersAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public string GetSecret(string appId, string endpointId, string idempotencyKey = default)
        {
            try
            {
                var lSecret = _endpointApi.GetEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet(
                    endpointId,
                    appId,
                    idempotencyKey);

                return lSecret?.Key;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetSecret)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<string> GetSecretAsync(string appId, string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lSecret = await _endpointApi.GetEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGetAsync(
                    endpointId,
                    appId,
                    idempotencyKey,
                    cancellationToken);

                return lSecret.Key;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetSecretAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public List<EndpointOut> List(string appId, ListOptions options = null, string idempotencyKey = default)
        {
            try
            {
                var lEndpoints = _endpointApi.ListEndpointsApiV1AppAppIdEndpointGet(
                    appId,
                    options?.Iterator,
                    options?.Limit,
                    options?.Order,
                    idempotencyKey);

                return lEndpoints?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;

                return new List<EndpointOut>();
            }
        }

        public async Task<List<EndpointOut>> ListAsync(string appId, ListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lEndpoints = await _endpointApi.ListEndpointsApiV1AppAppIdEndpointGetAsync(
                    appId,
                    options?.Iterator,
                    options?.Limit,
                    options?.Order,
                    idempotencyKey,
                    cancellationToken);

                return lEndpoints?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;

                return new List<EndpointOut>();
            }
        }

        public bool PatchHeaders(string appId, string endpointId, EndpointHeadersPatchIn headers, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _endpointApi.PatchEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPatchWithHttpInfo(
                    appId,
                    endpointId,
                    headers,
                    idempotencyKey);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(PatchHeaders)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public async Task<bool> PatchHeadersAsync(string appId, string endpointId, EndpointHeadersPatchIn headers, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _endpointApi.PatchEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPatchWithHttpInfoAsync(
                    appId,
                    endpointId,
                    headers,
                    idempotencyKey,
                    cancellationToken);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(PatchHeadersAsync)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public bool Recover(string appId, string endpointId, RecoverIn recover, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _endpointApi.RecoverFailedWebhooksApiV1AppAppIdEndpointEndpointIdRecoverPostWithHttpInfo(
                    appId,
                    endpointId,
                    recover,
                    idempotencyKey);

                return lResponse.StatusCode == HttpStatusCode.Accepted;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Recover)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public async Task<bool> RecoverAsync(string appId, string endpointId, RecoverIn recover, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _endpointApi.RecoverFailedWebhooksApiV1AppAppIdEndpointEndpointIdRecoverPostWithHttpInfoAsync(
                    appId,
                    endpointId,
                    recover,
                    idempotencyKey,
                    cancellationToken);

                return lResponse.StatusCode == HttpStatusCode.Accepted;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(RecoverAsync)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public bool RotateSecret(string appId, string endpointId, EndpointSecretRotateIn secret, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _endpointApi.RotateEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretRotatePostWithHttpInfo(
                    endpointId,
                    appId,
                    secret,
                    idempotencyKey);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(RotateSecret)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public async Task<bool> RotateSecretAsync(string appId, string endpointId, EndpointSecretRotateIn secret, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _endpointApi.RotateEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretRotatePostWithHttpInfoAsync(
                    endpointId,
                    appId,
                    secret,
                    idempotencyKey);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(RotateSecretAsync)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public EndpointOut Update(string appId, string endpointId, EndpointUpdate endpoint, string idempotencyKey = default)
        {
            try
            {
                var lEndpoint = _endpointApi.UpdateEndpointApiV1AppAppIdEndpointEndpointIdPut(
                    endpointId,
                    appId,
                    endpoint,
                    idempotencyKey);

                return lEndpoint;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Update)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<EndpointOut> UpdateAsync(string appId, string endpointId, EndpointUpdate endpoint, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lEndpoint = await _endpointApi.UpdateEndpointApiV1AppAppIdEndpointEndpointIdPutAsync(
                    endpointId,
                    appId,
                    endpoint,
                    idempotencyKey,
                    cancellationToken);

                return lEndpoint;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(UpdateAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public bool UpdateHeaders(string appId, string endpointId, EndpointHeadersIn headers, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _endpointApi.UpdateEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPutWithHttpInfo(
                    appId,
                    endpointId,
                    headers,
                    idempotencyKey);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(UpdateHeaders)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public async Task<bool> UpdateHeadersAsync(string appId, string endpointId, EndpointHeadersIn headers, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _endpointApi.UpdateEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPutWithHttpInfoAsync(
                    appId,
                    endpointId,
                    headers,
                    idempotencyKey,
                    cancellationToken);

                return lResponse.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(UpdateHeadersAsync)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public EndpointStats GetStats(string appId, string endpointId, string idempotencyKey = default)
        {
            try
            {
                var lStats = _endpointApi.GetEndpointStatsApiV1AppAppIdEndpointEndpointIdStatsGet(
                    endpointId,
                    appId,
                    idempotencyKey);

                return lStats;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetStats)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<EndpointStats> GetStatsAsync(string appId, string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lStats = await _endpointApi.GetEndpointStatsApiV1AppAppIdEndpointEndpointIdStatsGetAsync(
                    endpointId,
                    appId,
                    idempotencyKey,
                    cancellationToken);

                return lStats;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetStatsAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public bool ReplayMissing(string appId, string endpointId, ReplayIn replayIn,
            string idempotencyKey = default)
        {
            try
            {
                var response = _endpointApi.ReplayMissingWebhooksApiV1AppAppIdEndpointEndpointIdReplayMissingPostWithHttpInfo(
                    appId,
                    endpointId,
                    replayIn,
                    idempotencyKey);

                return response.StatusCode == HttpStatusCode.Accepted;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ReplayMissing)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public async Task<bool> ReplayMissingAsync(string appId, string endpointId, ReplayIn replayIn,
            string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var response = await _endpointApi.ReplayMissingWebhooksApiV1AppAppIdEndpointEndpointIdReplayMissingPostWithHttpInfoAsync(
                    appId,
                    endpointId,
                    replayIn,
                    idempotencyKey,
                    cancellationToken);

                return response.StatusCode == HttpStatusCode.Accepted;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ReplayMissingAsync)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public EndpointTransformationOut TransformationGet(string appId, string endpointId, string idempotencyKey = default)
        {
            try
            {
                var lTransformation = _endpointApi.GetEndpointTransformationApiV1AppAppIdEndpointEndpointIdTransformationGet(
                    appId,
                    endpointId,
                    idempotencyKey);

                return lTransformation;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(TransformationGet)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<EndpointTransformationOut> TransformationGetAsync(string appId, string endpointId, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lTransformation = await _endpointApi.GetEndpointTransformationApiV1AppAppIdEndpointEndpointIdTransformationGetAsync(
                    appId,
                    endpointId,
                    idempotencyKey,
                    cancellationToken);

                return lTransformation;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(TransformationGetAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public bool TransformationPartialUpdate(string appId, string endpointId, EndpointTransformationIn endpointTransformationIn, string idempotencyKey = default)
        {
            try
            {
                var response = _endpointApi.SetEndpointTransformationApiV1AppAppIdEndpointEndpointIdTransformationPatchWithHttpInfo(
                    appId,
                    endpointId,
                    endpointTransformationIn,
                    idempotencyKey);

                return response.StatusCode == HttpStatusCode.NoContent;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(TransformationGet)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public async Task<bool> TransformationPartialUpdateAsync(string appId, string endpointId, EndpointTransformationIn endpointTransformationIn, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var response = await _endpointApi.SetEndpointTransformationApiV1AppAppIdEndpointEndpointIdTransformationPatchWithHttpInfoAsync(
                    appId,
                    endpointId,
                    endpointTransformationIn,
                    idempotencyKey,
                    cancellationToken);

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
    }
}
