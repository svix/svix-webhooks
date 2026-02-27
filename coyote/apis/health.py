# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    PingOut,
)


class HealthAsync(ApiBase):
    async def ping(
        self,
    ) -> PingOut:
        """Verify the server is up and running."""
        return await self._request_asyncio(
            method="get",
            path="/api/v1/health/ping",
            response_type=PingOut,
        )


class Health(ApiBase):
    def ping(
        self,
    ) -> PingOut:
        """Verify the server is up and running."""
        return self._request_sync(
            method="get",
            path="/api/v1/health/ping",
            response_type=PingOut,
        )
