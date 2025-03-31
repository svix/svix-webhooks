# This file is @generated
import typing as t
from dataclasses import dataclass

from .. import models
from ..models import (
    ApiTokenExpireIn,
    ApiTokenIn,
    ApiTokenOut,
    ListResponseApiTokenCensoredOut,
)
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class ManagementAuthenticationListApiTokensOptions(BaseOptions):
    limit: t.Optional[int] = None
    """Limit the number of returned items"""
    iterator: t.Optional[str] = None
    """The iterator returned from a prior invocation"""
    order: t.Optional[models.Ordering] = None
    """The sorting order of the returned items"""

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "limit": self.limit,
                "iterator": self.iterator,
                "order": self.order,
            }
        )


@dataclass
class ManagementAuthenticationCreateApiTokenOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class ManagementAuthenticationExpireApiTokenOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class ManagementAuthenticationAsync(ApiBase):
    async def list_api_tokens(
        self,
        options: ManagementAuthenticationListApiTokensOptions = ManagementAuthenticationListApiTokensOptions(),
    ) -> ListResponseApiTokenCensoredOut:
        """List all API Tokens."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/management/authentication/api-token",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseApiTokenCensoredOut.model_validate(response.json())

    async def create_api_token(
        self,
        api_token_in: ApiTokenIn,
        options: ManagementAuthenticationCreateApiTokenOptions = ManagementAuthenticationCreateApiTokenOptions(),
    ) -> ApiTokenOut:
        """Create a new API Token."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/management/authentication/api-token",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=api_token_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return ApiTokenOut.model_validate(response.json())

    async def expire_api_token(
        self,
        key_id: str,
        api_token_expire_in: ApiTokenExpireIn,
        options: ManagementAuthenticationExpireApiTokenOptions = ManagementAuthenticationExpireApiTokenOptions(),
    ) -> None:
        """Expire the selected API Token."""
        await self._request_asyncio(
            method="post",
            path="/api/v1/management/authentication/api-token/{key_id}/expire",
            path_params={
                "key_id": key_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=api_token_expire_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )


class ManagementAuthentication(ApiBase):
    def list_api_tokens(
        self,
        options: ManagementAuthenticationListApiTokensOptions = ManagementAuthenticationListApiTokensOptions(),
    ) -> ListResponseApiTokenCensoredOut:
        """List all API Tokens."""
        response = self._request_sync(
            method="get",
            path="/api/v1/management/authentication/api-token",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseApiTokenCensoredOut.model_validate(response.json())

    def create_api_token(
        self,
        api_token_in: ApiTokenIn,
        options: ManagementAuthenticationCreateApiTokenOptions = ManagementAuthenticationCreateApiTokenOptions(),
    ) -> ApiTokenOut:
        """Create a new API Token."""
        response = self._request_sync(
            method="post",
            path="/api/v1/management/authentication/api-token",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=api_token_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return ApiTokenOut.model_validate(response.json())

    def expire_api_token(
        self,
        key_id: str,
        api_token_expire_in: ApiTokenExpireIn,
        options: ManagementAuthenticationExpireApiTokenOptions = ManagementAuthenticationExpireApiTokenOptions(),
    ) -> None:
        """Expire the selected API Token."""
        self._request_sync(
            method="post",
            path="/api/v1/management/authentication/api-token/{key_id}/expire",
            path_params={
                "key_id": key_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=api_token_expire_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
