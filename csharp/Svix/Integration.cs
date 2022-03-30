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
    public sealed class Integration : SvixResourceBase, IIntegration
    {
        private readonly IIntegrationApi _integrationApi;
        
        public Integration(ISvixClient svixClient, IIntegrationApi integrationApi) : base(svixClient)
        {
            _integrationApi = integrationApi ?? throw new ArgumentNullException(nameof(integrationApi));
        }

        public IntegrationOut Create(string appId, IntegrationIn integration, string idempotencyKey = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Create)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<IntegrationOut> CreateAsync(string appId, IntegrationIn integration, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public bool Delete(string appId, string integrationId, string idempotencyKey = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Delete)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public async Task<bool> DeleteAsync(string appId, string integrationId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(DeleteAsync)} failed");

                if (Throw)
                    throw;

                return false;
            }
        }

        public IntegrationOut Get(string appId, string integrationId, string idempotencyKey = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Get)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<IntegrationOut> GetAsync(string appId, string integrationId, string idempotencyKey = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public string GetKey(string appId, string integrationId, string idempotencyKey = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetKey)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<string> GetKeyAsync(string appId, string integrationId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(GetKeyAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public List<IntegrationOut> List(string appId, ListOptions options, string idempotencyKey = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(List)} failed");

                if (Throw)
                    throw;

                return new List<IntegrationOut>();
            }
        }

        public async Task<List<IntegrationOut>> ListAsync(string appId, ListOptions options, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                if (Throw)
                    throw;

                return new List<IntegrationOut>();
            }
        }

        public string RotateKey(string appId, string integrationId, string idempotencyKey = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(RotateKey)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<string> RotateKeyAsync(string appId, string integrationId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(RotateKeyAsync)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public IntegrationOut Update(string appId, string integrationId, IntegrationIn integration, string idempotencyKey = default)
        {
            try
            {
                throw new NotImplementedException();
            }
            catch (ApiException e)
            {
                Logger?.LogError(e, $"{nameof(Update)} failed");

                if (Throw)
                    throw;

                return null;
            }
        }

        public async Task<IntegrationOut> UpdateAsync(string appId, string integrationId, IntegrationIn integration, string idempotencyKey = default)
        {
            try
            {
                throw new NotImplementedException();
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