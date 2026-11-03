# This file is @generated
import typing as t
from dataclasses import dataclass

from ..models import McpEventsSubscribeIn, McpEventsSubscribeOut, McpEventsUnsubscribeIn
from .common import ApiBaseAsync, ApiBaseSync, BaseOptions, serialize_params


@dataclass
class EndpointMcpEventsSubscribeOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class EndpointMcpEventsUnsubscribeOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class EndpointMcpEventsAsync(ApiBaseAsync):
    async def subscribe(
        self,
        app_id: str,
        mcp_events_subscribe_in: McpEventsSubscribeIn,
        options: EndpointMcpEventsSubscribeOptions = (
            EndpointMcpEventsSubscribeOptions()
        ),
    ) -> McpEventsSubscribeOut:
        """Create or refresh an MCP Events subscription."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/mcp-events/subscribe",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=mcp_events_subscribe_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return McpEventsSubscribeOut.model_validate(response.json())

    async def unsubscribe(
        self,
        app_id: str,
        mcp_events_unsubscribe_in: McpEventsUnsubscribeIn,
        options: EndpointMcpEventsUnsubscribeOptions = (
            EndpointMcpEventsUnsubscribeOptions()
        ),
    ) -> None:
        """Stop an MCP Events subscription.

        Forward an MCP server's `events/unsubscribe` requests here. The
        subscription's endpoint is deleted. Unsubscribing from a subscription that
        doesn't exist succeeds, so this can be repeated."""
        await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/mcp-events/unsubscribe",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=mcp_events_unsubscribe_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )


class EndpointMcpEvents(ApiBaseSync):
    def subscribe(
        self,
        app_id: str,
        mcp_events_subscribe_in: McpEventsSubscribeIn,
        options: EndpointMcpEventsSubscribeOptions = (
            EndpointMcpEventsSubscribeOptions()
        ),
    ) -> McpEventsSubscribeOut:
        """Create or refresh an MCP Events subscription."""
        response = self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/mcp-events/subscribe",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=mcp_events_subscribe_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return McpEventsSubscribeOut.model_validate(response.json())

    def unsubscribe(
        self,
        app_id: str,
        mcp_events_unsubscribe_in: McpEventsUnsubscribeIn,
        options: EndpointMcpEventsUnsubscribeOptions = (
            EndpointMcpEventsUnsubscribeOptions()
        ),
    ) -> None:
        """Stop an MCP Events subscription.

        Forward an MCP server's `events/unsubscribe` requests here. The
        subscription's endpoint is deleted. Unsubscribing from a subscription that
        doesn't exist succeeds, so this can be repeated."""
        self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/mcp-events/unsubscribe",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=mcp_events_unsubscribe_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
