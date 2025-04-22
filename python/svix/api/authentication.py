# This file is @generated
import typing as t
from dataclasses import dataclass

from deprecated import deprecated

from ..models import (
    ApplicationTokenExpireIn,
    AppPortalAccessIn,
    AppPortalAccessOut,
    DashboardAccessOut,
)
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class AuthenticationAppPortalAccessOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class AuthenticationExpireAllOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class AuthenticationLogoutOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class AuthenticationDashboardAccessOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class AuthenticationAsync(ApiBase):
    async def app_portal_access(
        self,
        app_id: str,
        app_portal_access_in: AppPortalAccessIn,
        options: AuthenticationAppPortalAccessOptions = AuthenticationAppPortalAccessOptions(),
    ) -> AppPortalAccessOut:
        """Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/auth/app-portal-access/{app_id}",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=app_portal_access_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return AppPortalAccessOut.model_validate(response.json())

    async def expire_all(
        self,
        app_id: str,
        application_token_expire_in: ApplicationTokenExpireIn,
        options: AuthenticationExpireAllOptions = AuthenticationExpireAllOptions(),
    ) -> None:
        """Expire all of the tokens associated with a specific application."""
        await self._request_asyncio(
            method="post",
            path="/api/v1/auth/app/{app_id}/expire-all",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=application_token_expire_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )

    @deprecated
    async def dashboard_access(
        self,
        app_id: str,
        options: AuthenticationDashboardAccessOptions = AuthenticationDashboardAccessOptions(),
    ) -> DashboardAccessOut:
        """Deprecated: Please use `app_portal_access` instead."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/auth/dashboard-access/{app_id}",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return DashboardAccessOut.model_validate(response.json())

    async def logout(
        self, options: AuthenticationLogoutOptions = AuthenticationLogoutOptions()
    ) -> None:
        """Logout an app token.

        Trying to log out other tokens will fail."""
        await self._request_asyncio(
            method="post",
            path="/api/v1/auth/logout",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )


class Authentication(ApiBase):
    def app_portal_access(
        self,
        app_id: str,
        app_portal_access_in: AppPortalAccessIn,
        options: AuthenticationAppPortalAccessOptions = AuthenticationAppPortalAccessOptions(),
    ) -> AppPortalAccessOut:
        """Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal."""
        response = self._request_sync(
            method="post",
            path="/api/v1/auth/app-portal-access/{app_id}",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=app_portal_access_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return AppPortalAccessOut.model_validate(response.json())

    def expire_all(
        self,
        app_id: str,
        application_token_expire_in: ApplicationTokenExpireIn,
        options: AuthenticationExpireAllOptions = AuthenticationExpireAllOptions(),
    ) -> None:
        """Expire all of the tokens associated with a specific application."""
        self._request_sync(
            method="post",
            path="/api/v1/auth/app/{app_id}/expire-all",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=application_token_expire_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )

    @deprecated
    def dashboard_access(
        self,
        app_id: str,
        options: AuthenticationDashboardAccessOptions = AuthenticationDashboardAccessOptions(),
    ) -> DashboardAccessOut:
        """Deprecated: Please use `app_portal_access` instead."""
        response = self._request_sync(
            method="post",
            path="/api/v1/auth/dashboard-access/{app_id}",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return DashboardAccessOut.model_validate(response.json())

    def logout(
        self, options: AuthenticationLogoutOptions = AuthenticationLogoutOptions()
    ) -> None:
        """Logout an app token.

        Trying to log out other tokens will fail."""
        self._request_sync(
            method="post",
            path="/api/v1/auth/logout",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
