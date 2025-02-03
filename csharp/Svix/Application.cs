// this file is @generated
#nullable enable
using Svix.Models;

namespace Svix
{
    public class ApplicationListOptions : SvixOptionsBase
    {
        public ulong? Limit { get; set; }
        public string? Iterator { get; set; }
        public Ordering? Order { get; set; }

        public new Dictionary<string, string> QueryParams()
        {
            return SerializeParams(
                new Dictionary<string, object?>
                {
                    { "limit", Limit },
                    { "iterator", Iterator },
                    { "order", Order },
                }
            );
        }
    }

    public class ApplicationCreateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class Application(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// List of all the organization's applications.
        /// </summary>
        public async Task<ListResponseApplicationOut> ListAsync(
            ApplicationListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            var response =
                await this._client.SvixHttpClient.SendRequestAsync<ListResponseApplicationOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    cancellationToken: cancellationToken
                );
            return response.Data;
        }

        /// <summary>
        /// List of all the organization's applications.
        /// </summary>
        public ListResponseApplicationOut List(ApplicationListOptions? options = null)
        {
            var response = this._client.SvixHttpClient.SendRequest<ListResponseApplicationOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app",
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams()
            );
            return response.Data;
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
            applicationIn = applicationIn ?? throw new ArgumentNullException(nameof(applicationIn));

            var response = await this._client.SvixHttpClient.SendRequestAsync<ApplicationOut>(
                method: HttpMethod.Post,
                path: "/api/v1/app",
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: applicationIn,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Create a new application.
        /// </summary>
        public ApplicationOut Create(
            ApplicationIn applicationIn,
            ApplicationCreateOptions? options = null
        )
        {
            applicationIn = applicationIn ?? throw new ArgumentNullException(nameof(applicationIn));

            var response = this._client.SvixHttpClient.SendRequest<ApplicationOut>(
                method: HttpMethod.Post,
                path: "/api/v1/app",
                queryParams: options?.QueryParams(),
                headerParams: options?.HeaderParams(),
                content: applicationIn
            );
            return response.Data;
        }

        /// <summary>
        /// Get an application.
        /// </summary>
        public async Task<ApplicationOut> GetAsync(
            string appId,
            CancellationToken cancellationToken = default
        )
        {
            var response = await this._client.SvixHttpClient.SendRequestAsync<ApplicationOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Get an application.
        /// </summary>
        public ApplicationOut Get(string appId)
        {
            var response = this._client.SvixHttpClient.SendRequest<ApplicationOut>(
                method: HttpMethod.Get,
                path: "/api/v1/app/{app_id}",
                pathParams: new Dictionary<string, string> { { "app_id", appId } }
            );
            return response.Data;
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
            applicationIn = applicationIn ?? throw new ArgumentNullException(nameof(applicationIn));

            var response = await this._client.SvixHttpClient.SendRequestAsync<ApplicationOut>(
                method: HttpMethod.Put,
                path: "/api/v1/app/{app_id}",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                content: applicationIn,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Update an application.
        /// </summary>
        public ApplicationOut Update(string appId, ApplicationIn applicationIn)
        {
            applicationIn = applicationIn ?? throw new ArgumentNullException(nameof(applicationIn));

            var response = this._client.SvixHttpClient.SendRequest<ApplicationOut>(
                method: HttpMethod.Put,
                path: "/api/v1/app/{app_id}",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                content: applicationIn
            );
            return response.Data;
        }

        /// <summary>
        /// Delete an application.
        /// </summary>
        public async Task<bool> DeleteAsync(
            string appId,
            CancellationToken cancellationToken = default
        )
        {
            var response = await this._client.SvixHttpClient.SendRequestAsync<bool>(
                method: HttpMethod.Delete,
                path: "/api/v1/app/{app_id}",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Delete an application.
        /// </summary>
        public bool Delete(string appId)
        {
            var response = this._client.SvixHttpClient.SendRequest<bool>(
                method: HttpMethod.Delete,
                path: "/api/v1/app/{app_id}",
                pathParams: new Dictionary<string, string> { { "app_id", appId } }
            );
            return response.Data;
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
            applicationPatch =
                applicationPatch ?? throw new ArgumentNullException(nameof(applicationPatch));

            var response = await this._client.SvixHttpClient.SendRequestAsync<ApplicationOut>(
                method: HttpMethod.Patch,
                path: "/api/v1/app/{app_id}",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                content: applicationPatch,
                cancellationToken: cancellationToken
            );
            return response.Data;
        }

        /// <summary>
        /// Partially update an application.
        /// </summary>
        public ApplicationOut Patch(string appId, ApplicationPatch applicationPatch)
        {
            applicationPatch =
                applicationPatch ?? throw new ArgumentNullException(nameof(applicationPatch));

            var response = this._client.SvixHttpClient.SendRequest<ApplicationOut>(
                method: HttpMethod.Patch,
                path: "/api/v1/app/{app_id}",
                pathParams: new Dictionary<string, string> { { "app_id", appId } },
                content: applicationPatch
            );
            return response.Data;
        }
    }
}
