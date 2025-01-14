from .common import PostOptions, ApiBase

from ..internal.openapi_client.api.authentication import (
    v1_authentication_app_portal_access,
    v1_authentication_dashboard_access,
    v1_authentication_logout,
)
from ..internal.openapi_client.models.app_portal_access_in import AppPortalAccessIn
from ..internal.openapi_client.models.app_portal_access_out import AppPortalAccessOut
from ..internal.openapi_client.models.dashboard_access_out import DashboardAccessOut


class AuthenticationAsync(ApiBase):
    async def app_portal_access(
        self,
        app_id: str,
        app_portal_access_in: AppPortalAccessIn,
        options: PostOptions = PostOptions(),
    ) -> AppPortalAccessOut:
        return await v1_authentication_app_portal_access.request_asyncio(
            client=self._client,
            app_id=app_id,
            json_body=app_portal_access_in,
            **options.to_dict(),
        )

    async def dashboard_access(
        self, app_id: str, options: PostOptions = PostOptions()
    ) -> DashboardAccessOut:
        return await v1_authentication_dashboard_access.request_asyncio(
            client=self._client, app_id=app_id, **options.to_dict()
        )

    async def logout(self, options: PostOptions = PostOptions()) -> None:
        return await v1_authentication_logout.request_asyncio(
            client=self._client, **options.to_dict()
        )


class Authentication(ApiBase):
    def app_portal_access(
        self,
        app_id: str,
        app_portal_access_in: AppPortalAccessIn,
        options: PostOptions = PostOptions(),
    ) -> AppPortalAccessOut:
        return v1_authentication_app_portal_access.request_sync(
            client=self._client,
            app_id=app_id,
            json_body=app_portal_access_in,
            **options.to_dict(),
        )

    def dashboard_access(
        self, app_id: str, options: PostOptions = PostOptions()
    ) -> DashboardAccessOut:
        return v1_authentication_dashboard_access.request_sync(
            client=self._client, app_id=app_id, **options.to_dict()
        )

    def logout(self, options: PostOptions = PostOptions()) -> None:
        return v1_authentication_logout.request_sync(
            client=self._client, **options.to_dict()
        )
