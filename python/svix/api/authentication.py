import typing as t
from dataclasses import dataclass
from deprecated import deprecated

from .common import ApiBase, BaseOptions


from ..internal.openapi_client.api.authentication import (
    v1_authentication_app_portal_access,
    v1_authentication_dashboard_access,
    v1_authentication_expire_all,
    v1_authentication_logout,
)

from ..internal.openapi_client.models.app_portal_access_in import AppPortalAccessIn
from ..internal.openapi_client.models.app_portal_access_out import AppPortalAccessOut
from ..internal.openapi_client.models.dashboard_access_out import DashboardAccessOut
from ..internal.openapi_client.models.application_token_expire_in import (
    ApplicationTokenExpireIn,
)


@dataclass
class AuthenticationAppPortalAccessOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


@dataclass
class AuthenticationDashboardAccessOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


@dataclass
class AuthenticationExpireAllOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


@dataclass
class AuthenticationLogoutOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


class AuthenticationAsync(ApiBase):
    async def app_portal_access(
        self,
        app_id: str,
        app_portal_access_in: AppPortalAccessIn,
        options: AuthenticationAppPortalAccessOptions = AuthenticationAppPortalAccessOptions(),
    ) -> AppPortalAccessOut:
        """Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal."""
        return await v1_authentication_app_portal_access.request_asyncio(
            client=self._client,
            app_id=app_id,
            json_body=app_portal_access_in,
            **options.to_dict(),
        )

    @deprecated
    async def dashboard_access(
        self,
        app_id: str,
        options: AuthenticationDashboardAccessOptions = AuthenticationDashboardAccessOptions(),
    ) -> DashboardAccessOut:
        return await v1_authentication_dashboard_access.request_asyncio(
            client=self._client, app_id=app_id, **options.to_dict()
        )

    async def expire_all(
        self,
        app_id: str,
        application_token_expire_in: ApplicationTokenExpireIn,
        options: AuthenticationExpireAllOptions = AuthenticationExpireAllOptions(),
    ) -> None:
        """Expire all of the tokens associated with a specific application."""
        return await v1_authentication_expire_all.request_asyncio(
            client=self._client,
            app_id=app_id,
            json_body=application_token_expire_in,
            **options.to_dict(),
        )

    async def logout(
        self, options: AuthenticationLogoutOptions = AuthenticationLogoutOptions()
    ) -> None:
        """Logout an app token.

        Trying to log out other tokens will fail."""
        return await v1_authentication_logout.request_asyncio(
            client=self._client, **options.to_dict()
        )


class Authentication(ApiBase):
    def app_portal_access(
        self,
        app_id: str,
        app_portal_access_in: AppPortalAccessIn,
        options: AuthenticationAppPortalAccessOptions = AuthenticationAppPortalAccessOptions(),
    ) -> AppPortalAccessOut:
        """Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal."""
        return v1_authentication_app_portal_access.request_sync(
            client=self._client,
            app_id=app_id,
            json_body=app_portal_access_in,
            **options.to_dict(),
        )

    def expire_all(
        self,
        app_id: str,
        application_token_expire_in: ApplicationTokenExpireIn,
        options: AuthenticationExpireAllOptions = AuthenticationExpireAllOptions(),
    ) -> None:
        """Expire all of the tokens associated with a specific application."""
        return v1_authentication_expire_all.request_sync(
            client=self._client,
            app_id=app_id,
            json_body=application_token_expire_in,
            **options.to_dict(),
        )

    @deprecated
    def dashboard_access(
        self,
        app_id: str,
        options: AuthenticationDashboardAccessOptions = AuthenticationDashboardAccessOptions(),
    ) -> DashboardAccessOut:
        return v1_authentication_dashboard_access.request_sync(
            client=self._client, app_id=app_id, **options.to_dict()
        )

    def logout(
        self, options: AuthenticationLogoutOptions = AuthenticationLogoutOptions()
    ) -> None:
        """Logout an app token.

        Trying to log out other tokens will fail."""
        return v1_authentication_logout.request_sync(
            client=self._client, **options.to_dict()
        )
