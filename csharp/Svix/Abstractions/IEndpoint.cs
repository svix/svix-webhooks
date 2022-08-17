using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Svix.Model;
using Svix.Models;

namespace Svix.Abstractions
{
    public interface IEndpoint
    {
        EndpointOut Create(string appId, EndpointIn endpoint, string idempotencyKey = default);

        Task<EndpointOut> CreateAsync(string appId, EndpointIn endpoint, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        bool Delete(string appId, string endpointId, string idempotencyKey = default);

        Task<bool> DeleteAsync(string appId, string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        EndpointOut Get(string appId, string endpointId, string idempotencyKey = default);

        Task<EndpointOut> GetAsync(string appId, string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        EndpointHeadersOut GetHeaders(string appId, string endpointId, string idempotencyKey = default);

        Task<EndpointHeadersOut> GetHeadersAsync(string appId, string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        string GetSecret(string appId, string endpointId, string idempotencyKey = default);

        Task<string> GetSecretAsync(string appId, string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        List<EndpointOut> List(string appId, ListOptions options = null, string idempotencyKey = default);

        Task<List<EndpointOut>> ListAsync(string appId, ListOptions options = null, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        bool PatchHeaders(string appId, string endpointId, EndpointHeadersPatchIn headers, string idempotencyKey = default);

        Task<bool> PatchHeadersAsync(string appId, string endpointId, EndpointHeadersPatchIn headers,
            string idempotencyKey = default, CancellationToken cancellationToken = default);

        bool Recover(string appId, string endpointId, RecoverIn recover, string idempotencyKey = default);

        Task<bool> RecoverAsync(string appId, string endpointId, RecoverIn recover, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        bool RotateSecret(string appId, string endpointId, EndpointSecretRotateIn secret, string idempotencyKey = default);

        Task<bool> RotateSecretAsync(string appId, string endpointId, EndpointSecretRotateIn secret, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        EndpointOut Update(string appId, string endpointId, EndpointUpdate endpoint, string idempotencyKey = default);

        Task<EndpointOut> UpdateAsync(string appId, string endpointId, EndpointUpdate endpoint, string idempotencyKey = default,
            CancellationToken cancellationToken = default);

        bool UpdateHeaders(string appId, string endpointId, EndpointHeadersIn headers, string idempotencyKey = default);

        Task<bool> UpdateHeadersAsync(string appId, string endpointId, EndpointHeadersIn headers,
            string idempotencyKey = default, CancellationToken cancellationToken = default);

        EndpointStats GetStats(string appId, string endpointId, string idempotencyKey = default);

        Task<EndpointStats> GetStatsAsync(string appId, string endpointId, string idempotencyKey = default,
            CancellationToken cancellationToken = default);
    }
}
