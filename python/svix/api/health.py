# This file is @generated
from .common import ApiBase


class HealthAsync(ApiBase):
    async def get(self) -> None:
        """Verify the API server is up and running."""
        await self._request_asyncio(method="get", path="/api/v1/health", path_params={})


class Health(ApiBase):
    def get(self) -> None:
        """Verify the API server is up and running."""
        self._request_sync(method="get", path="/api/v1/health", path_params={})
