using System;
using System.Collections.Generic;
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

        public ApplicationOut Create(ApplicationIn application, ApplicationCreateOptions options, string idempotencyKey = default)
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
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }
        
        public async Task<ApplicationOut> CreateAsync(ApplicationIn application, ApplicationCreateOptions options, string idempotencyKey = default, CancellationToken cancellationToken = default)
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
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public ApplicationOut Delete()
        {
            throw new NotImplementedException();
        }
        
        public async Task<ApplicationOut> DeleteAsync()
        {
            throw new NotImplementedException();
        }
        
        public ApplicationOut Get()
        {
            throw new NotImplementedException();
        }
        
        public async Task<ApplicationOut> GetAsync()
        {
            throw new NotImplementedException();
        }
        
        public List<ApplicationOut> List(ApplicationListOptions options = null, string idempotencyKey = default)
        {
            try
            {
                var lResponse = _applicationApi.ListApplicationsApiV1AppGet(
                    options?.Iterator,
                    options?.Limit,
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

        public async Task<List<ApplicationOut>> ListAsync(ApplicationListOptions options, string idempotencyKey = default, CancellationToken cancellationToken = default)
        {
            try
            {
                var lResponse = await _applicationApi.ListApplicationsApiV1AppGetAsync(
                    options?.Iterator,
                    options?.Limit,
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
        
        public ApplicationOut Update()
        {
            throw new NotImplementedException();
        }
        
        public async Task<ApplicationOut> UpdateAsync()
        {
            throw new NotImplementedException();
        }
    }
}