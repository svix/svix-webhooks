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

                var lApplication = _applicationApi.CreateApplicationApiV1AppPost(
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

                var lApplication = await _applicationApi.CreateApplicationApiV1AppPostAsync(
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
                var lResponse = _applicationApi.DeleteApplicationApiV1AppAppIdDeleteWithHttpInfo(
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

        public async Task<bool> DeleteAsync(string appId, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _applicationApi.DeleteApplicationApiV1AppAppIdDeleteWithHttpInfoAsync(
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

        public ApplicationOut Get(string appId, string idempotencyKey = default)
        {
            try
            {
                var lApplication = _applicationApi.GetApplicationApiV1AppAppIdGet(
                    appId,
                    idempotencyKey);

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
                var lApplication = await _applicationApi.GetApplicationApiV1AppAppIdGetAsync(
                    appId,
                    idempotencyKey);

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

        public List<ApplicationOut> List(ListOptions options = null, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _applicationApi.ListApplicationsApiV1AppGet(
                    options?.Iterator,
                    options?.Limit,
                    options?.Order,
                    idempotencyKey);

                return lResponse?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;

                return new List<ApplicationOut>();
            }
        }

        public async Task<List<ApplicationOut>> ListAsync(ListOptions options = null, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _applicationApi.ListApplicationsApiV1AppGetAsync(
                    options?.Iterator,
                    options?.Limit,
                    options?.Order,
                    idempotencyKey,
                    cancellationToken);

                return lResponse?.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;

                return new List<ApplicationOut>();
            }
        }

        public ApplicationOut Update(string appId, ApplicationIn application, string idempotencyKey = default)
        {
            try
            {
                var lApplication = _applicationApi.UpdateApplicationApiV1AppAppIdPut(
                    appId,
                    application,
                    idempotencyKey);

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
                var lApplication = await _applicationApi.UpdateApplicationApiV1AppAppIdPutAsync(
                    appId,
                    application,
                    idempotencyKey,
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
    }
}
