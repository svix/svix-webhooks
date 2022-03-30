using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Svix.Abstractions;
using Svix.Api;
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
            throw new System.NotImplementedException();
        }

        public Task<IntegrationOut> CreateAsync(string appId, IntegrationIn integration, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            throw new System.NotImplementedException();
        }

        public bool Delete(string appId, string integrationId, string idempotencyKey = default)
        {
            throw new System.NotImplementedException();
        }

        public Task<bool> DeleteAsync(string appId, string integrationId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            throw new System.NotImplementedException();
        }

        public IntegrationOut Get(string appId, string integrationId, string idempotencyKey = default)
        {
            throw new System.NotImplementedException();
        }

        public Task<IntegrationOut> GetAsync(string appId, string integrationId, string idempotencyKey = default)
        {
            throw new System.NotImplementedException();
        }

        public string GetKey(string appId, string integrationId, string idempotencyKey = default)
        {
            throw new System.NotImplementedException();
        }

        public Task<string> GetKeyAsync(string appId, string integrationId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            throw new System.NotImplementedException();
        }

        public List<IntegrationOut> List(string appId, ListOptions options, string idempotencyKey = default)
        {
            throw new System.NotImplementedException();
        }

        public Task<List<IntegrationOut>> ListAsync(string appId, ListOptions options, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            throw new System.NotImplementedException();
        }

        public string RotateKey(string appId, string integrationId, string idempotencyKey = default)
        {
            throw new System.NotImplementedException();
        }

        public Task<string> RotateKeyAsync(string appId, string integrationId, string idempotencyKey = default,
            CancellationToken cancellationToken = default)
        {
            throw new System.NotImplementedException();
        }

        public IntegrationOut Update(string appId, string integrationId, IntegrationIn integration, string idempotencyKey = default)
        {
            throw new System.NotImplementedException();
        }

        public Task<IntegrationOut> UpdateAsync(string appId, string integrationId, IntegrationIn integration, string idempotencyKey = default)
        {
            throw new System.NotImplementedException();
        }
    }
}