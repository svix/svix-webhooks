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
                var lEndpoint = _endpointApi.V1EndpointCreate(
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
                var lEndpoint = await _endpointApi.V1EndpointCreateAsync(
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
                var lResponse = _endpointApi.V1EndpointDeleteWithHttpInfo(
                    appId,
                    endpointId);

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
                var lResponse = await _endpointApi.V1EndpointDeleteWithHttpInfoAsync(
                    appId,
                    endpointId,
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
                var lEndpoint = _endpointApi.V1EndpointGet(
                    appId,
                    endpointId);

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
                var lEndpoint = await _endpointApi.V1EndpointGetAsync(
                    appId,
                    endpointId,
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
                var lHeaders = _endpointApi.V1EndpointGetHeaders(
                    appId,
                    endpointId);

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
                var lHeaders = await _endpointApi.V1EndpointGetHeadersAsync(
                    appId,
                    endpointId,
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
                var lSecret = _endpointApi.V1EndpointGetSecret(
                    appId,
                    endpointId);

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
                var lSecret = await _endpointApi.V1EndpointGetSecretAsync(
                    appId,
                    endpointId,
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

        public ListResponseEndpointOut List(string appId, ListOptions options = null, string idempotencyKey = default)
        {
            try
            {
                var lEndpoints = _endpointApi.V1EndpointList(
                    appId,
                    options?.Limit,
                    options?.Iterator,
                    options?.Order);

                return lEndpoints;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;

                return new ListResponseEndpointOut();
            }
        }

        public async Task<ListResponseEndpointOut> ListAsync(string appId, ListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lEndpoints = await _endpointApi.V1EndpointListAsync(
                    appId,
                    options?.Limit,
                    options?.Iterator,
                    options?.Order,
                    cancellationToken);

                return lEndpoints;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;

                return new ListResponseEndpointOut();
            }
        }

        public bool PatchHeaders(string appId, string endpointId, EndpointHeadersPatchIn headers, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _endpointApi.V1EndpointPatchHeadersWithHttpInfo(
                    appId,
                    endpointId,
                    headers);

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
                var lResponse = await _endpointApi.V1EndpointPatchHeadersWithHttpInfoAsync(
                    appId,
                    endpointId,
                    headers,
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
                var lResponse = _endpointApi.V1EndpointRecoverWithHttpInfo(
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
                var lResponse = await _endpointApi.V1EndpointRecoverWithHttpInfoAsync(
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
                var lResponse = _endpointApi.V1EndpointRotateSecretWithHttpInfo(
                    appId,
                    endpointId,
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
                var lResponse = await _endpointApi.V1EndpointRotateSecretWithHttpInfoAsync(
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
                var lEndpoint = _endpointApi.V1EndpointUpdate(
                    appId,
                    endpointId,
                    endpoint);

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
                var lEndpoint = await _endpointApi.V1EndpointUpdateAsync(
                    appId,
                    endpointId,
                    endpoint,
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

        public EndpointOut Patch(string appId, string endpointId, EndpointPatch endpoint, string idempotencyKey = default)
        {
            try
            {
                var lEndpoint = _endpointApi.V1EndpointPatch(
                    appId,
                    endpointId,
                    endpoint);

                return lEndpoint;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Patch)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<EndpointOut> PatchAsync(string appId, string endpointId, EndpointPatch endpoint, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lEndpoint = await _endpointApi.V1EndpointPatchAsync(
                    appId,
                    endpointId,
                    endpoint,
                    cancellationToken);

                return lEndpoint;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(PatchAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public bool UpdateHeaders(string appId, string endpointId, EndpointHeadersIn headers, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _endpointApi.V1EndpointUpdateHeadersWithHttpInfo(
                    appId,
                    endpointId,
                    headers);

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
                var lResponse = await _endpointApi.V1EndpointUpdateHeadersWithHttpInfoAsync(
                    appId,
                    endpointId,
                    headers,
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
                var lStats = _endpointApi.V1EndpointGetStats(
                    appId,
                    endpointId,
                    null,
                    null);

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

        public EndpointStats GetStatsWithOptions(string appId, string endpointId, EndpointStatsOptions options = null, string idempotencyKey = default)
        {
            try
            {
                var lStats = _endpointApi.V1EndpointGetStats(
                    appId,
                    endpointId,
                    options?.Since,
                    options?.Until);

                return lStats;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetStatsWithOptions)} failed");

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
                var lStats = await _endpointApi.V1EndpointGetStatsAsync(
                    appId,
                    endpointId,
                    null,
                    null,
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

        public async Task<EndpointStats> GetStatsWithOptionsAsync(string appId, string endpointId, EndpointStatsOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                var lStats = await _endpointApi.V1EndpointGetStatsAsync(
                    appId,
                    endpointId,
                    options?.Since,
                    options?.Until,
                    cancellationToken);

                return lStats;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetStatsWithOptionsAsync)} failed");

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
                var response = _endpointApi.V1EndpointReplayWithHttpInfo(
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
                var response = await _endpointApi.V1EndpointReplayWithHttpInfoAsync(
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
                var lTransformation = _endpointApi.V1EndpointTransformationGet(
                    appId,
                    endpointId);

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
                var lTransformation = await _endpointApi.V1EndpointTransformationGetAsync(
                    appId,
                    endpointId,
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
                var response = _endpointApi.V1EndpointTransformationPartialUpdateWithHttpInfo(
                    appId,
                    endpointId,
                    endpointTransformationIn);

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
                var response = await _endpointApi.V1EndpointTransformationPartialUpdateWithHttpInfoAsync(
                    appId,
                    endpointId,
                    endpointTransformationIn,
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

        public MessageOut SendExample(string appId, string endpointId, EventExampleIn eventExampleIn, string idempotencyKey = default)
        {
            try
            {
                var response = _endpointApi.V1EndpointSendExample(
                    appId,
                    endpointId,
                    eventExampleIn,
                    idempotencyKey);

                return response;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(SendExample)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<MessageOut> SendExampleAsync(string appId, string endpointId, EventExampleIn eventExampleIn, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var response = await _endpointApi.V1EndpointSendExampleAsync(
                    appId,
                    endpointId,
                    eventExampleIn,
                    idempotencyKey);

                return response;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(SendExampleAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }
    }
}
