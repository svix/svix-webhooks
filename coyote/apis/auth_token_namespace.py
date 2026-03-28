# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    AuthTokenCreateNamespaceIn,
    AuthTokenCreateNamespaceOut,
    AuthTokenGetNamespaceIn,
    AuthTokenGetNamespaceOut,
)


class AuthTokenNamespaceAsync(ApiBase):
    async def create(
        self,
        auth_token_create_namespace_in: AuthTokenCreateNamespaceIn,
    ) -> AuthTokenCreateNamespaceOut:
        """Create Auth Token namespace"""
        body = auth_token_create_namespace_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.auth-token.namespace.create",
            body=body,
            response_type=AuthTokenCreateNamespaceOut,
        )

    async def get(
        self,
        auth_token_get_namespace_in: AuthTokenGetNamespaceIn,
    ) -> AuthTokenGetNamespaceOut:
        """Get Auth Token namespace"""
        body = auth_token_get_namespace_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.auth-token.namespace.get",
            body=body,
            response_type=AuthTokenGetNamespaceOut,
        )


class AuthTokenNamespace(ApiBase):
    def create(
        self,
        auth_token_create_namespace_in: AuthTokenCreateNamespaceIn,
    ) -> AuthTokenCreateNamespaceOut:
        """Create Auth Token namespace"""
        body = auth_token_create_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.auth-token.namespace.create",
            body=body,
            response_type=AuthTokenCreateNamespaceOut,
        )

    def get(
        self,
        auth_token_get_namespace_in: AuthTokenGetNamespaceIn,
    ) -> AuthTokenGetNamespaceOut:
        """Get Auth Token namespace"""
        body = auth_token_get_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.auth-token.namespace.get",
            body=body,
            response_type=AuthTokenGetNamespaceOut,
        )
