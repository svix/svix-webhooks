using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Svix.Model;
using Svix.Models;

namespace Svix.Abstractions
{
    public interface IOperationalWebhookEndpoint
    {
        OperationalWebhookEndpointOut Create(OperationalWebhookEndpointIn endpoint,
            string idempotencyKey = default);

        Task<OperationalWebhookEndpointOut> CreateAsync(OperationalWebhookEndpointIn endpoint,
            string idempotencyKey = default, CancellationToken cancellationToken = default);

        bool Delete(string endpointId, string idempotencyKey = default);

        Task<bool> DeleteAsync(string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        OperationalWebhookEndpointOut Get(string endpointId, string idempotencyKey = default);

        Task<OperationalWebhookEndpointOut> GetAsync(string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        string GetSecret(string endpointId, string idempotencyKey = default);

        Task<string> GetSecretAsync(string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        ListResponseOperationalWebhookEndpointOut List(ListOptions options = null,
            string idempotencyKey = default);

        Task<ListResponseOperationalWebhookEndpointOut> ListAsync(ListOptions options = null,
            string idempotencyKey = default, CancellationToken cancellationToken = default);

        bool RotateSecret(string endpointId, OperationalWebhookEndpointSecretIn secret,
            string idempotencyKey = default);

        Task<bool> RotateSecretAsync(string endpointId, OperationalWebhookEndpointSecretIn secret,
            string idempotencyKey = default, CancellationToken cancellationToken = default);

        OperationalWebhookEndpointOut Update(string endpointId,
            OperationalWebhookEndpointUpdate endpoint, string idempotencyKey = default);

        Task<OperationalWebhookEndpointOut> UpdateAsync(string endpointId,
            OperationalWebhookEndpointUpdate endpoint, string idempotencyKey = default,
            CancellationToken cancellationToken = default);
    }
}
