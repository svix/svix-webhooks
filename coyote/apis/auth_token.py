# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    AuthTokenCreateIn,
    AuthTokenCreateOut,
    AuthTokenDeleteIn,
    AuthTokenDeleteOut,
    AuthTokenExpireIn,
    AuthTokenExpireOut,
    AuthTokenListIn,
    AuthTokenRotateIn,
    AuthTokenRotateOut,
    AuthTokenUpdateIn,
    AuthTokenUpdateOut,
    AuthTokenVerifyIn,
    AuthTokenVerifyOut,
    ListResponseAuthTokenOut,
)
from .auth_token_namespace import (
    AuthTokenNamespace,
    AuthTokenNamespaceAsync,
)


class AuthTokenAsync(ApiBase):
    @property
    def namespace(self) -> AuthTokenNamespaceAsync:
        return AuthTokenNamespaceAsync(self._client)

    async def create(
        self,
        auth_token_create_in: AuthTokenCreateIn,
    ) -> AuthTokenCreateOut:
        """Create Auth Token"""
        body = auth_token_create_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.auth-token.create",
            body=body,
            response_type=AuthTokenCreateOut,
        )

    async def expire(
        self,
        auth_token_expire_in: AuthTokenExpireIn,
    ) -> AuthTokenExpireOut:
        """Expire Auth Token"""
        body = auth_token_expire_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.auth-token.expire",
            body=body,
            response_type=AuthTokenExpireOut,
        )

    async def delete(
        self,
        auth_token_delete_in: AuthTokenDeleteIn,
    ) -> AuthTokenDeleteOut:
        """Delete Auth Token"""
        body = auth_token_delete_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.auth-token.delete",
            body=body,
            response_type=AuthTokenDeleteOut,
        )

    async def verify(
        self,
        auth_token_verify_in: AuthTokenVerifyIn,
    ) -> AuthTokenVerifyOut:
        """Verify Auth Token"""
        body = auth_token_verify_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.auth-token.verify",
            body=body,
            response_type=AuthTokenVerifyOut,
        )

    async def list(
        self,
        auth_token_list_in: AuthTokenListIn,
    ) -> ListResponseAuthTokenOut:
        """List Auth Tokens"""
        body = auth_token_list_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.auth-token.list",
            body=body,
            response_type=ListResponseAuthTokenOut,
        )

    async def update(
        self,
        auth_token_update_in: AuthTokenUpdateIn,
    ) -> AuthTokenUpdateOut:
        """Update Auth Token"""
        body = auth_token_update_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.auth-token.update",
            body=body,
            response_type=AuthTokenUpdateOut,
        )

    async def rotate(
        self,
        auth_token_rotate_in: AuthTokenRotateIn,
    ) -> AuthTokenRotateOut:
        """Rotate Auth Token"""
        body = auth_token_rotate_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.auth-token.rotate",
            body=body,
            response_type=AuthTokenRotateOut,
        )


class AuthToken(ApiBase):
    @property
    def namespace(self) -> AuthTokenNamespace:
        return AuthTokenNamespace(self._client)

    def create(
        self,
        auth_token_create_in: AuthTokenCreateIn,
    ) -> AuthTokenCreateOut:
        """Create Auth Token"""
        body = auth_token_create_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.auth-token.create",
            body=body,
            response_type=AuthTokenCreateOut,
        )

    def expire(
        self,
        auth_token_expire_in: AuthTokenExpireIn,
    ) -> AuthTokenExpireOut:
        """Expire Auth Token"""
        body = auth_token_expire_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.auth-token.expire",
            body=body,
            response_type=AuthTokenExpireOut,
        )

    def delete(
        self,
        auth_token_delete_in: AuthTokenDeleteIn,
    ) -> AuthTokenDeleteOut:
        """Delete Auth Token"""
        body = auth_token_delete_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.auth-token.delete",
            body=body,
            response_type=AuthTokenDeleteOut,
        )

    def verify(
        self,
        auth_token_verify_in: AuthTokenVerifyIn,
    ) -> AuthTokenVerifyOut:
        """Verify Auth Token"""
        body = auth_token_verify_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.auth-token.verify",
            body=body,
            response_type=AuthTokenVerifyOut,
        )

    def list(
        self,
        auth_token_list_in: AuthTokenListIn,
    ) -> ListResponseAuthTokenOut:
        """List Auth Tokens"""
        body = auth_token_list_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.auth-token.list",
            body=body,
            response_type=ListResponseAuthTokenOut,
        )

    def update(
        self,
        auth_token_update_in: AuthTokenUpdateIn,
    ) -> AuthTokenUpdateOut:
        """Update Auth Token"""
        body = auth_token_update_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.auth-token.update",
            body=body,
            response_type=AuthTokenUpdateOut,
        )

    def rotate(
        self,
        auth_token_rotate_in: AuthTokenRotateIn,
    ) -> AuthTokenRotateOut:
        """Rotate Auth Token"""
        body = auth_token_rotate_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.auth-token.rotate",
            body=body,
            response_type=AuthTokenRotateOut,
        )
