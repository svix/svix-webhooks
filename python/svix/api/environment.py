# This file is @generated
import typing as t
from dataclasses import dataclass

from ..models import EnvironmentIn, EnvironmentOut
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class EnvironmentExportOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class EnvironmentImportOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class EnvironmentAsync(ApiBase):
    async def export(
        self, options: EnvironmentExportOptions = EnvironmentExportOptions()
    ) -> EnvironmentOut:
        """Download a JSON file containing all org-settings and event types."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/environment/export",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return EnvironmentOut.model_validate(response.json())

    async def import_(
        self,
        environment_in: EnvironmentIn,
        options: EnvironmentImportOptions = EnvironmentImportOptions(),
    ) -> None:
        """Import a configuration into the active organization.

        It doesn't delete anything, only adds / updates what was passed to it."""
        await self._request_asyncio(
            method="post",
            path="/api/v1/environment/import",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=environment_in.model_dump_json(exclude_unset=True, by_alias=True),
        )


class Environment(ApiBase):
    def export(
        self, options: EnvironmentExportOptions = EnvironmentExportOptions()
    ) -> EnvironmentOut:
        """Download a JSON file containing all org-settings and event types."""
        response = self._request_sync(
            method="post",
            path="/api/v1/environment/export",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return EnvironmentOut.model_validate(response.json())

    def import_(
        self,
        environment_in: EnvironmentIn,
        options: EnvironmentImportOptions = EnvironmentImportOptions(),
    ) -> None:
        """Import a configuration into the active organization.

        It doesn't delete anything, only adds / updates what was passed to it."""
        self._request_sync(
            method="post",
            path="/api/v1/environment/import",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=environment_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
