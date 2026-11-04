// this file is @generated
#nullable enable
using Microsoft.Extensions.Logging;
using Svix.Models;

namespace Svix
{
    public class EndpointMcpEventsSubscribeOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class EndpointMcpEventsUnsubscribeOptions : SvixOptionsBase
    {
        public string? IdempotencyKey { get; set; }

        public new Dictionary<string, string> HeaderParams()
        {
            return SerializeParams(
                new Dictionary<string, object?> { { "idempotency-key", IdempotencyKey } }
            );
        }
    }

    public class EndpointMcpEvents(SvixClient client)
    {
        readonly SvixClient _client = client;

        /// <summary>
        /// Create or refresh an MCP Events subscription.
        /// </summary>
        public async Task<McpEventsSubscribeOut> SubscribeAsync(
            string appId,
            McpEventsSubscribeIn mcpEventsSubscribeIn,
            EndpointMcpEventsSubscribeOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            if (options == null)
            {
                options = new EndpointMcpEventsSubscribeOptions();
            }
            mcpEventsSubscribeIn =
                mcpEventsSubscribeIn
                ?? throw new ArgumentNullException(nameof(mcpEventsSubscribeIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<McpEventsSubscribeOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/mcp-events/subscribe",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options.QueryParams(),
                    headerParams: options.HeaderParams(),
                    content: mcpEventsSubscribeIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(SubscribeAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Create or refresh an MCP Events subscription.
        /// </summary>
        public McpEventsSubscribeOut Subscribe(
            string appId,
            McpEventsSubscribeIn mcpEventsSubscribeIn,
            EndpointMcpEventsSubscribeOptions? options = null
        )
        {
            if (options == null)
            {
                options = new EndpointMcpEventsSubscribeOptions();
            }
            mcpEventsSubscribeIn =
                mcpEventsSubscribeIn
                ?? throw new ArgumentNullException(nameof(mcpEventsSubscribeIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<McpEventsSubscribeOut>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/mcp-events/subscribe",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options.QueryParams(),
                    headerParams: options.HeaderParams(),
                    content: mcpEventsSubscribeIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Subscribe)} failed");

                throw;
            }
        }

        /// <summary>
        /// Stop an MCP Events subscription.
        ///
        /// Forward an MCP server's `events/unsubscribe` requests here. The
        /// subscription's endpoint is deleted. Unsubscribing from a subscription that
        /// doesn't exist succeeds, so this can be repeated.
        /// </summary>
        public async Task<bool> UnsubscribeAsync(
            string appId,
            McpEventsUnsubscribeIn mcpEventsUnsubscribeIn,
            EndpointMcpEventsUnsubscribeOptions? options = null,
            CancellationToken cancellationToken = default
        )
        {
            if (options == null)
            {
                options = new EndpointMcpEventsUnsubscribeOptions();
            }
            mcpEventsUnsubscribeIn =
                mcpEventsUnsubscribeIn
                ?? throw new ArgumentNullException(nameof(mcpEventsUnsubscribeIn));
            try
            {
                var response = await _client.SvixHttpClient.SendRequestAsync<bool>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/mcp-events/unsubscribe",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options.QueryParams(),
                    headerParams: options.HeaderParams(),
                    content: mcpEventsUnsubscribeIn,
                    cancellationToken: cancellationToken
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(UnsubscribeAsync)} failed");

                throw;
            }
        }

        /// <summary>
        /// Stop an MCP Events subscription.
        ///
        /// Forward an MCP server's `events/unsubscribe` requests here. The
        /// subscription's endpoint is deleted. Unsubscribing from a subscription that
        /// doesn't exist succeeds, so this can be repeated.
        /// </summary>
        public bool Unsubscribe(
            string appId,
            McpEventsUnsubscribeIn mcpEventsUnsubscribeIn,
            EndpointMcpEventsUnsubscribeOptions? options = null
        )
        {
            if (options == null)
            {
                options = new EndpointMcpEventsUnsubscribeOptions();
            }
            mcpEventsUnsubscribeIn =
                mcpEventsUnsubscribeIn
                ?? throw new ArgumentNullException(nameof(mcpEventsUnsubscribeIn));
            try
            {
                var response = _client.SvixHttpClient.SendRequest<bool>(
                    method: HttpMethod.Post,
                    path: "/api/v1/app/{app_id}/mcp-events/unsubscribe",
                    pathParams: new Dictionary<string, string> { { "app_id", appId } },
                    queryParams: options.QueryParams(),
                    headerParams: options.HeaderParams(),
                    content: mcpEventsUnsubscribeIn
                );
                return response.Data;
            }
            catch (ApiException e)
            {
                _client.Logger?.LogError(e, $"{nameof(Unsubscribe)} failed");

                throw;
            }
        }
    }
}
