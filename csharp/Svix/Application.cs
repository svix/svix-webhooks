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
    public partial class ApplicationListOptions
    {
        public int? Limit { get; set; }
        public string? Iterator { get; set; }
        public Ordering? Order { get; set; }
    }

    public partial class ApplicationCreateOptions
    {
        public string? IdempotencyKey { get; set; }
    }

    public sealed class Application : SvixResourceBase
    {
        private readonly ApplicationApi _applicationApi;

        public Application(ISvixClient svixClient, ApplicationApi applicationApi)
            : base(svixClient)
        {
            _applicationApi =
                applicationApi ?? throw new ArgumentNullException(nameof(applicationApi));
        }

        /// <summary>
        /// List of all the organization's applications.
        /// </summary>
        public async Task<ListResponseApplicationOut> ListAsync(
            ApplicationListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _applicationApi.V1ApplicationListWithHttpInfoAsync(
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
                return new ListResponseApplicationOut();
            }
        }

        /// <summary>
        /// List of all the organization's applications.
        /// </summary>
        public ListResponseApplicationOut List(ApplicationListOptions? options = null)
        {
            try
            {
                var response = _applicationApi.V1ApplicationListWithHttpInfo(
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
                return new ListResponseApplicationOut();
            }
        }

        /// <summary>
        /// Create a new application.
        /// </summary>
        public async Task<ApplicationOut> CreateAsync(
            ApplicationIn applicationIn,
            ApplicationCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                applicationIn =
                    applicationIn ?? throw new ArgumentNullException(nameof(applicationIn));
                var response = await _applicationApi.V1ApplicationCreateWithHttpInfoAsync(
                    applicationIn: applicationIn,
                    idempotencyKey: options?.IdempotencyKey,
                    getIfExists: false,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                if (Throw)
                    throw;
                return new ApplicationOut();
            }
        }

        /// <summary>
        /// Create a new application.
        /// </summary>
        public ApplicationOut Create(
            ApplicationIn applicationIn,
            ApplicationCreateOptions? options = null
        )
        {
            try
            {
                applicationIn =
                    applicationIn ?? throw new ArgumentNullException(nameof(applicationIn));
                var response = _applicationApi.V1ApplicationCreateWithHttpInfo(
                    applicationIn: applicationIn,
                    idempotencyKey: options?.IdempotencyKey,
                    getIfExists: false
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Create)} failed");

                if (Throw)
                    throw;
                return new ApplicationOut();
            }
        }

        public async Task<ApplicationOut> GetOrCreateAsync(
            ApplicationIn applicationIn,
            ApplicationCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _applicationApi.V1ApplicationCreateWithHttpInfoAsync(
                    applicationIn: applicationIn,
                    idempotencyKey: options?.IdempotencyKey,
                    getIfExists: true,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                if (Throw)
                    throw;
                return new ApplicationOut();
            }
        }

        public ApplicationOut GetOrCreate(
            ApplicationIn applicationIn,
            ApplicationCreateOptions? options = null
        )
        {
            try
            {
                var response = _applicationApi.V1ApplicationCreateWithHttpInfo(
                    applicationIn: applicationIn,
                    idempotencyKey: options?.IdempotencyKey,
                    getIfExists: true
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Create)} failed");

                if (Throw)
                    throw;
                return new ApplicationOut();
            }
        }

        /// <summary>
        /// Get an application.
        /// </summary>
        public async Task<ApplicationOut> GetAsync(
            string appId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _applicationApi.V1ApplicationGetWithHttpInfoAsync(
                    appId: appId,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                if (Throw)
                    throw;
                return new ApplicationOut();
            }
        }

        /// <summary>
        /// Get an application.
        /// </summary>
        public ApplicationOut Get(string appId)
        {
            try
            {
                var response = _applicationApi.V1ApplicationGetWithHttpInfo(appId: appId);
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Get)} failed");

                if (Throw)
                    throw;
                return new ApplicationOut();
            }
        }

        /// <summary>
        /// Update an application.
        /// </summary>
        public async Task<ApplicationOut> UpdateAsync(
            string appId,
            ApplicationIn applicationIn,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                applicationIn =
                    applicationIn ?? throw new ArgumentNullException(nameof(applicationIn));
                var response = await _applicationApi.V1ApplicationUpdateWithHttpInfoAsync(
                    appId: appId,
                    applicationIn: applicationIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(UpdateAsync)} failed");

                if (Throw)
                    throw;
                return new ApplicationOut();
            }
        }

        /// <summary>
        /// Update an application.
        /// </summary>
        public ApplicationOut Update(string appId, ApplicationIn applicationIn)
        {
            try
            {
                applicationIn =
                    applicationIn ?? throw new ArgumentNullException(nameof(applicationIn));
                var response = _applicationApi.V1ApplicationUpdateWithHttpInfo(
                    appId: appId,
                    applicationIn: applicationIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Update)} failed");

                if (Throw)
                    throw;
                return new ApplicationOut();
            }
        }

        /// <summary>
        /// Delete an application.
        /// </summary>
        public async Task<bool> DeleteAsync(
            string appId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _applicationApi.V1ApplicationDeleteWithHttpInfoAsync(
                    appId: appId,
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
        /// Delete an application.
        /// </summary>
        public bool Delete(string appId)
        {
            try
            {
                var response = _applicationApi.V1ApplicationDeleteWithHttpInfo(appId: appId);
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
        /// Partially update an application.
        /// </summary>
        public async Task<ApplicationOut> PatchAsync(
            string appId,
            ApplicationPatch applicationPatch,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                applicationPatch =
                    applicationPatch ?? throw new ArgumentNullException(nameof(applicationPatch));
                var response = await _applicationApi.V1ApplicationPatchWithHttpInfoAsync(
                    appId: appId,
                    applicationPatch: applicationPatch,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(PatchAsync)} failed");

                if (Throw)
                    throw;
                return new ApplicationOut();
            }
        }

        /// <summary>
        /// Partially update an application.
        /// </summary>
        public ApplicationOut Patch(string appId, ApplicationPatch applicationPatch)
        {
            try
            {
                applicationPatch =
                    applicationPatch ?? throw new ArgumentNullException(nameof(applicationPatch));
                var response = _applicationApi.V1ApplicationPatchWithHttpInfo(
                    appId: appId,
                    applicationPatch: applicationPatch
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Patch)} failed");

                if (Throw)
                    throw;
                return new ApplicationOut();
            }
        }
    }
}
