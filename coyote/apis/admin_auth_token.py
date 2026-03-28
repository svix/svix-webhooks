# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    AdminAuthTokenCreateIn,
    AdminAuthTokenCreateOut,
    AdminAuthTokenDeleteIn,
    AdminAuthTokenDeleteOut,
    AdminAuthTokenExpireIn,
    AdminAuthTokenExpireOut,
    AdminAuthTokenListIn,
    AdminAuthTokenRotateIn,
    AdminAuthTokenRotateOut,
    AdminAuthTokenUpdateIn,
    AdminAuthTokenUpdateOut,
    AdminAuthTokenWhoamiIn,
    AdminAuthTokenWhoamiOut,
    ListResponseAdminAuthTokenOut,
)


class AdminAuthTokenAsync(ApiBase):
    async def create(
        self,
        admin_auth_token_create_in: AdminAuthTokenCreateIn,
    ) -> AdminAuthTokenCreateOut:
        """Create an auth token"""
        body = admin_auth_token_create_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-token.create",
            body=body,
            response_type=AdminAuthTokenCreateOut,
        )

    async def expire(
        self,
        admin_auth_token_expire_in: AdminAuthTokenExpireIn,
    ) -> AdminAuthTokenExpireOut:
        """Expire an auth token"""
        body = admin_auth_token_expire_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-token.expire",
            body=body,
            response_type=AdminAuthTokenExpireOut,
        )

    async def rotate(
        self,
        admin_auth_token_rotate_in: AdminAuthTokenRotateIn,
    ) -> AdminAuthTokenRotateOut:
        """Rotate an auth token, invalidating the old one and issuing a new secret"""
        body = admin_auth_token_rotate_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-token.rotate",
            body=body,
            response_type=AdminAuthTokenRotateOut,
        )

    async def delete(
        self,
        admin_auth_token_delete_in: AdminAuthTokenDeleteIn,
    ) -> AdminAuthTokenDeleteOut:
        """Delete an auth token"""
        body = admin_auth_token_delete_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-token.delete",
            body=body,
            response_type=AdminAuthTokenDeleteOut,
        )

    async def list(
        self,
        admin_auth_token_list_in: AdminAuthTokenListIn = AdminAuthTokenListIn(),
    ) -> ListResponseAdminAuthTokenOut:
        """List auth tokens for a given owner"""
        body = admin_auth_token_list_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-token.list",
            body=body,
            response_type=ListResponseAdminAuthTokenOut,
        )

    async def update(
        self,
        admin_auth_token_update_in: AdminAuthTokenUpdateIn,
    ) -> AdminAuthTokenUpdateOut:
        """Update an auth token's properties"""
        body = admin_auth_token_update_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-token.update",
            body=body,
            response_type=AdminAuthTokenUpdateOut,
        )

    async def whoami(
        self,
        admin_auth_token_whoami_in: AdminAuthTokenWhoamiIn = AdminAuthTokenWhoamiIn(),
    ) -> AdminAuthTokenWhoamiOut:
        """Return the role of the currently authenticated token"""
        body = admin_auth_token_whoami_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-token.whoami",
            body=body,
            response_type=AdminAuthTokenWhoamiOut,
        )


class AdminAuthToken(ApiBase):
    def create(
        self,
        admin_auth_token_create_in: AdminAuthTokenCreateIn,
    ) -> AdminAuthTokenCreateOut:
        """Create an auth token"""
        body = admin_auth_token_create_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.admin.auth-token.create",
            body=body,
            response_type=AdminAuthTokenCreateOut,
        )

    def expire(
        self,
        admin_auth_token_expire_in: AdminAuthTokenExpireIn,
    ) -> AdminAuthTokenExpireOut:
        """Expire an auth token"""
        body = admin_auth_token_expire_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.admin.auth-token.expire",
            body=body,
            response_type=AdminAuthTokenExpireOut,
        )

    def rotate(
        self,
        admin_auth_token_rotate_in: AdminAuthTokenRotateIn,
    ) -> AdminAuthTokenRotateOut:
        """Rotate an auth token, invalidating the old one and issuing a new secret"""
        body = admin_auth_token_rotate_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.admin.auth-token.rotate",
            body=body,
            response_type=AdminAuthTokenRotateOut,
        )

    def delete(
        self,
        admin_auth_token_delete_in: AdminAuthTokenDeleteIn,
    ) -> AdminAuthTokenDeleteOut:
        """Delete an auth token"""
        body = admin_auth_token_delete_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.admin.auth-token.delete",
            body=body,
            response_type=AdminAuthTokenDeleteOut,
        )

    def list(
        self,
        admin_auth_token_list_in: AdminAuthTokenListIn = AdminAuthTokenListIn(),
    ) -> ListResponseAdminAuthTokenOut:
        """List auth tokens for a given owner"""
        body = admin_auth_token_list_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.admin.auth-token.list",
            body=body,
            response_type=ListResponseAdminAuthTokenOut,
        )

    def update(
        self,
        admin_auth_token_update_in: AdminAuthTokenUpdateIn,
    ) -> AdminAuthTokenUpdateOut:
        """Update an auth token's properties"""
        body = admin_auth_token_update_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.admin.auth-token.update",
            body=body,
            response_type=AdminAuthTokenUpdateOut,
        )

    def whoami(
        self,
        admin_auth_token_whoami_in: AdminAuthTokenWhoamiIn = AdminAuthTokenWhoamiIn(),
    ) -> AdminAuthTokenWhoamiOut:
        """Return the role of the currently authenticated token"""
        body = admin_auth_token_whoami_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.admin.auth-token.whoami",
            body=body,
            response_type=AdminAuthTokenWhoamiOut,
        )
