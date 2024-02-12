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
    public sealed class Application : SvixResourceBase, IApplication
    {
        private readonly IApplicationApi _applicationApi;

        public Application(ISvixClient svixClient, IApplicationApi applicationApi)
            : base(svixClient)
        {
            _applicationApi = applicationApi ?? throw new ArgumentNullException(nameof(applicationApi));
        }

        public ApplicationOut Create(ApplicationIn application, ApplicationCreateOptions options = null, string idempotencyKey = default)
        {
            try
            {
                application = application ?? throw new ArgumentNullException(nameof(application));

                var lApplication = _applicationApi.V1ApplicationCreate(
                    application,
                    options?.GetIfExists ?? false,
                    idempotencyKey);

                return lApplication;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Create)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<ApplicationOut> CreateAsync(ApplicationIn application, ApplicationCreateOptions options = null, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                application = application ?? throw new ArgumentNullException(nameof(application));

                var lApplication = await _applicationApi.V1ApplicationCreateAsync(
                    application,
                    options?.GetIfExists ?? false,
                    idempotencyKey,
                    cancellationToken);

                return lApplication;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public bool Delete(string appId, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _applicationApi.V1ApplicationDeleteWithHttpInfo(appId);

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

        public async Task<bool> DeleteAsync(string appId, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _applicationApi.V1ApplicationDeleteWithHttpInfoAsync(
                    appId,
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

        public ApplicationOut Get(string appId, string idempotencyKey = default)
        {
            try
            {
                var lApplication = _applicationApi.V1ApplicationGet(appId);

                return lApplication;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Get)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<ApplicationOut> GetAsync(string appId, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lApplication = await _applicationApi.V1ApplicationGetAsync(appId);

                return lApplication;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public ListResponseApplicationOut List(ListOptions options = null, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _applicationApi.V1ApplicationList(
                    options?.Limit,
                    options?.Iterator,
                    options?.Order);

                return lResponse;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;

                return new ListResponseApplicationOut();
            }
        }

        public async Task<ListResponseApplicationOut> ListAsync(ListOptions options = null, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _applicationApi.V1ApplicationListAsync(
                    options?.Limit,
                    options?.Iterator,
                    options?.Order,
                    cancellationToken);

                return lResponse;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;

                return new ListResponseApplicationOut();
            }
        }

        public ApplicationOut Update(string appId, ApplicationIn application, string idempotencyKey = default)
        {
            try
            {
                var lApplication = _applicationApi.V1ApplicationUpdate(
                    appId,
                    application);

                return lApplication;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Update)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<ApplicationOut> UpdateAsync(string appId, ApplicationIn application, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lApplication = await _applicationApi.V1ApplicationUpdateAsync(
                    appId,
                    application,
                    cancellationToken);

                return lApplication;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(UpdateAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public ApplicationOut Patch(string appId, ApplicationPatch application, string idempotencyKey = default)
        {
            try
            {
                var lApplication = _applicationApi.V1ApplicationPatch(
                    appId,
                    application);

                return lApplication;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Patch)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<ApplicationOut> PatchAsync(string appId, ApplicationPatch application, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lApplication = await _applicationApi.V1ApplicationPatchAsync(
                    appId,
                    application,
                    cancellationToken);

                return lApplication;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(PatchAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }
    }
}
