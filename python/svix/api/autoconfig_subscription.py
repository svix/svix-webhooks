# This file is @generated
import typing as t
from dataclasses import dataclass

from ..models import AutoConfigOut, RotateSubscriptionIn2
from .common import ApiBaseAsync, ApiBaseSync, BaseOptions, serialize_params


@dataclass
class AutoconfigSubscriptionCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class AutoconfigSubscriptionRotateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class AutoconfigSubscriptionAsync(ApiBaseAsync):
    async def create(
        self,
        app_id: str,
        options: AutoconfigSubscriptionCreateOptions = (
            AutoconfigSubscriptionCreateOptions()
        ),
    ) -> AutoConfigOut:
        """Create an AutoConfig subscription."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/autoconfig",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return AutoConfigOut.model_validate(response.json())

    async def rotate(
        self,
        app_id: str,
        autoconfig_id: str,
        rotate_subscription_in2: RotateSubscriptionIn2,
        options: AutoconfigSubscriptionRotateOptions = (
            AutoconfigSubscriptionRotateOptions()
        ),
    ) -> AutoConfigOut:
        """Rotate the auth token and signing secret for an AutoConfig subscription."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/rotate",
            path_params={
                "app_id": app_id,
                "autoconfig_id": autoconfig_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=rotate_subscription_in2.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return AutoConfigOut.model_validate(response.json())


class AutoconfigSubscription(ApiBaseSync):
    def create(
        self,
        app_id: str,
        options: AutoconfigSubscriptionCreateOptions = (
            AutoconfigSubscriptionCreateOptions()
        ),
    ) -> AutoConfigOut:
        """Create an AutoConfig subscription."""
        response = self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/autoconfig",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return AutoConfigOut.model_validate(response.json())

    def rotate(
        self,
        app_id: str,
        autoconfig_id: str,
        rotate_subscription_in2: RotateSubscriptionIn2,
        options: AutoconfigSubscriptionRotateOptions = (
            AutoconfigSubscriptionRotateOptions()
        ),
    ) -> AutoConfigOut:
        """Rotate the auth token and signing secret for an AutoConfig subscription."""
        response = self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/rotate",
            path_params={
                "app_id": app_id,
                "autoconfig_id": autoconfig_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=rotate_subscription_in2.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return AutoConfigOut.model_validate(response.json())
