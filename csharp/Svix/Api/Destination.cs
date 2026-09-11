// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix.Api
{
    public class DestinationListOptions : SvixOptionsBase
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

    public class DestinationCreateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class Destination(SvixClient client)
    {
        readonly SvixClient _client = client;

        public DestinationTransformation Transformation
        {
            get => new DestinationTransformation(_client);
        }

        /// <summary>
        /// List of all the application's destinations.
        /// </summary>
        public async Task<ListResponseDestinationOut> ListAsync(
            string appId,
            DestinationListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            if (options == null)
            {
                options = new DestinationListOptions();
            }
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<ListResponseDestinationOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/app/{app_id}/destination",
                        pathParams: new Dictionary<string, string> { { "app_id", appId } },
                        queryParams: options.QueryParams(),
                        headerParams: options.HeaderParams(),
                        cancellationToken: cancellationToken
                    );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(ListAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// List of all the application's destinations.
        /// </summary>
        public ListResponseDestinationOut List(string appId, DestinationListOptions? options = null)
        {
            if (options == null)
            {
                options = new DestinationListOptions();
            }
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ListResponseDestinationOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/destination",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options.QueryParams(),
                    headerParams: options.HeaderParams()
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(List)} failed");

                throw;
            }
        }

        /// <summary>
        /// Creates a new destination.
        /// </summary>
        public async Task<DestinationOut> CreateAsync(
            string appId,
            DestinationIn destinationIn,
            DestinationCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            if (options == null)
            {
                options = new DestinationCreateOptions();
            }
            destinationIn = destinationIn ?? throw new ArgumentNullException(nameof(destinationIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<DestinationOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/destination",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options.QueryParams(),
                    headerParams: options.HeaderParams(),
                    content: destinationIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(CreateAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Creates a new destination.
        /// </summary>
        public DestinationOut Create(
            string appId,
            DestinationIn destinationIn,
            DestinationCreateOptions? options = null
        )
        {
            if (options == null)
            {
                options = new DestinationCreateOptions();
            }
            destinationIn = destinationIn ?? throw new ArgumentNullException(nameof(destinationIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<DestinationOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/destination",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options.QueryParams(),
                    headerParams: options.HeaderParams(),
                    content: destinationIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Create)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get a destination by id or uid.
        /// </summary>
        public async Task<DestinationOut> GetAsync(
            string appId,
            string destinationId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<DestinationOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/destination/{destination_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "destination_id", destinationId },
                    },
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(GetAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Get a destination by id or uid.
        /// </summary>
        public DestinationOut Get(string appId, string destinationId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<DestinationOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/app/{app_id}/destination/{destination_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "destination_id", destinationId },
                    }
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Get)} failed");

                throw;
            }
        }

        /// <summary>
        /// Create or update a destination.
        /// </summary>
        public async Task<DestinationOut> UpsertAsync(
            string appId,
            string destinationId,
            DestinationIn destinationIn,
            CancellationToken cancellationToken = default
        )
        {
            destinationIn = destinationIn ?? throw new ArgumentNullException(nameof(destinationIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<DestinationOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/app/{app_id}/destination/{destination_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "destination_id", destinationId },
                    },
                    content: destinationIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(UpsertAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Create or update a destination.
        /// </summary>
        public DestinationOut Upsert(
            string appId,
            string destinationId,
            DestinationIn destinationIn
        )
        {
            destinationIn = destinationIn ?? throw new ArgumentNullException(nameof(destinationIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<DestinationOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/app/{app_id}/destination/{destination_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "destination_id", destinationId },
                    },
                    content: destinationIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Upsert)} failed");

                throw;
            }
        }

        /// <summary>
        /// Delete a destination.
        /// </summary>
        public async Task<bool> DeleteAsync(
            string appId,
            string destinationId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/app/{app_id}/destination/{destination_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "destination_id", destinationId },
                    },
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(DeleteAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Delete a destination.
        /// </summary>
        public bool Delete(string appId, string destinationId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/app/{app_id}/destination/{destination_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "destination_id", destinationId },
                    }
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Delete)} failed");

                throw;
            }
        }

        /// <summary>
        /// Partially update a destination.
        /// </summary>
        public async Task<DestinationOut> PatchAsync(
            string appId,
            string destinationId,
            DestinationPatch destinationPatch,
            CancellationToken cancellationToken = default
        )
        {
            destinationPatch =
                destinationPatch ?? throw new ArgumentNullException(nameof(destinationPatch));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<DestinationOut>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/app/{app_id}/destination/{destination_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "destination_id", destinationId },
                    },
                    content: destinationPatch,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(PatchAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Partially update a destination.
        /// </summary>
        public DestinationOut Patch(
            string appId,
            string destinationId,
            DestinationPatch destinationPatch
        )
        {
            destinationPatch =
                destinationPatch ?? throw new ArgumentNullException(nameof(destinationPatch));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<DestinationOut>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/app/{app_id}/destination/{destination_id}",
                    pathParams: new Dictionary<string, string>
                    {
                        { "app_id", appId },
                        { "destination_id", destinationId },
                    },
                    content: destinationPatch
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Patch)} failed");

                throw;
            }
        }
    }
}
