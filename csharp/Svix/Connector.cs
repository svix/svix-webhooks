// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class ConnectorListOptions : SvixOptionsBase
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

    public class ConnectorCreateOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class Connector(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// List all connectors for an application.
        /// </summary>
        public async Task<ListResponseConnectorOut> ListAsync(
            ConnectorListOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response =
                    await _client.SvixHttpClient.SendRequestAsync<ListResponseConnectorOut>(
                        method: HttpMethod.Get,
                        path: "/api/v1/connector",
                        queryParams: options?.QueryParams(),
                        headerParams: options?.HeaderParams(),
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
        /// List all connectors for an application.
        /// </summary>
        public ListResponseConnectorOut List(ConnectorListOptions? options = null)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ListResponseConnectorOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/connector",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams()
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
        /// Create a new connector.
        /// </summary>
        public async Task<ConnectorOut> CreateAsync(
            ConnectorIn connectorIn,
            ConnectorCreateOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            connectorIn = connectorIn ?? throw new ArgumentNullException(nameof(connectorIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<ConnectorOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/connector",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: connectorIn,
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
        /// Create a new connector.
        /// </summary>
        public ConnectorOut Create(ConnectorIn connectorIn, ConnectorCreateOptions? options = null)
        {
            connectorIn = connectorIn ?? throw new ArgumentNullException(nameof(connectorIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ConnectorOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/connector",
                    queryParams: options?.QueryParams(),
                    headerParams: options?.HeaderParams(),
                    content: connectorIn
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
        /// Get a connector.
        /// </summary>
        public async Task<ConnectorOut> GetAsync(
            string connectorId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<ConnectorOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/connector/{connector_id}",
                    pathParams: new Dictionary<string, string> { { "connector_id", connectorId } },
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
        /// Get a connector.
        /// </summary>
        public ConnectorOut Get(string connectorId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ConnectorOut>(
                    method: HttpMethod.Get,
                    path: "/api/v1/connector/{connector_id}",
                    pathParams: new Dictionary<string, string> { { "connector_id", connectorId } }
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
        /// Update a connector.
        /// </summary>
        public async Task<ConnectorOut> UpdateAsync(
            string connectorId,
            ConnectorUpdate connectorUpdate,
            CancellationToken cancellationToken = default
        )
        {
            connectorUpdate =
                connectorUpdate ?? throw new ArgumentNullException(nameof(connectorUpdate));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<ConnectorOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/connector/{connector_id}",
                    pathParams: new Dictionary<string, string> { { "connector_id", connectorId } },
                    content: connectorUpdate,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(UpdateAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Update a connector.
        /// </summary>
        public ConnectorOut Update(string connectorId, ConnectorUpdate connectorUpdate)
        {
            connectorUpdate =
                connectorUpdate ?? throw new ArgumentNullException(nameof(connectorUpdate));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ConnectorOut>(
                    method: HttpMethod.Put,
                    path: "/api/v1/connector/{connector_id}",
                    pathParams: new Dictionary<string, string> { { "connector_id", connectorId } },
                    content: connectorUpdate
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Update)} failed");

                throw;
            }
        }

        /// <summary>
        /// Delete a connector.
        /// </summary>
        public async Task<bool> DeleteAsync(
            string connectorId,
            CancellationToken cancellationToken = default
        )
        {
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/connector/{connector_id}",
                    pathParams: new Dictionary<string, string> { { "connector_id", connectorId } },
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
        /// Delete a connector.
        /// </summary>
        public bool Delete(string connectorId)
        {
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Delete,
                    path: "/api/v1/connector/{connector_id}",
                    pathParams: new Dictionary<string, string> { { "connector_id", connectorId } }
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
        /// Partially update a connector.
        /// </summary>
        public async Task<ConnectorOut> PatchAsync(
            string connectorId,
            ConnectorPatch connectorPatch,
            CancellationToken cancellationToken = default
        )
        {
            connectorPatch =
                connectorPatch ?? throw new ArgumentNullException(nameof(connectorPatch));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<ConnectorOut>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/connector/{connector_id}",
                    pathParams: new Dictionary<string, string> { { "connector_id", connectorId } },
                    content: connectorPatch,
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
        /// Partially update a connector.
        /// </summary>
        public ConnectorOut Patch(string connectorId, ConnectorPatch connectorPatch)
        {
            connectorPatch =
                connectorPatch ?? throw new ArgumentNullException(nameof(connectorPatch));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<ConnectorOut>(
                    method: HttpMethod.Patch,
                    path: "/api/v1/connector/{connector_id}",
                    pathParams: new Dictionary<string, string> { { "connector_id", connectorId } },
                    content: connectorPatch
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
