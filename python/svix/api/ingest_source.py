# This file is @generated
import typing as t
from dataclasses import dataclass

from .. import models
from ..models import (
    IngestSourceIn,
    IngestSourceOut,
    ListResponseIngestSourceOut,
    RotateTokenOut,
)
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class IngestSourceListOptions(BaseOptions):
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
class IngestSourceCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class IngestSourceRotateTokenOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class IngestSourceAsync(ApiBase):
    async def list(
        self, options: IngestSourceListOptions = IngestSourceListOptions()
    ) -> ListResponseIngestSourceOut:
        """List of all the organization's Ingest Sources."""
        response = await self._request_asyncio(
            method="get",
            path="/ingest/api/v1/source",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseIngestSourceOut.model_validate(response.json())

    async def create(
        self,
        ingest_source_in: IngestSourceIn,
        options: IngestSourceCreateOptions = IngestSourceCreateOptions(),
    ) -> IngestSourceOut:
        """Create Ingest Source."""
        response = await self._request_asyncio(
            method="post",
            path="/ingest/api/v1/source",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=ingest_source_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return IngestSourceOut.model_validate(response.json())

    async def get(self, source_id: str) -> IngestSourceOut:
        """Get an Ingest Source by id or uid."""
        response = await self._request_asyncio(
            method="get",
            path="/ingest/api/v1/source/{source_id}",
            path_params={
                "source_id": source_id,
            },
        )
        return IngestSourceOut.model_validate(response.json())

    async def update(
        self, source_id: str, ingest_source_in: IngestSourceIn
    ) -> IngestSourceOut:
        """Update an Ingest Source."""
        response = await self._request_asyncio(
            method="put",
            path="/ingest/api/v1/source/{source_id}",
            path_params={
                "source_id": source_id,
            },
            json_body=ingest_source_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return IngestSourceOut.model_validate(response.json())

    async def delete(self, source_id: str) -> None:
        """Delete an Ingest Source."""
        await self._request_asyncio(
            method="delete",
            path="/ingest/api/v1/source/{source_id}",
            path_params={
                "source_id": source_id,
            },
        )

    async def rotate_token(
        self,
        source_id: str,
        options: IngestSourceRotateTokenOptions = IngestSourceRotateTokenOptions(),
    ) -> RotateTokenOut:
        """Rotate the Ingest Source's Url Token.

        This will rotate the ingest source's token, which is used to
        construct the unique `ingestUrl` for the source. Previous tokens
        will remain valid for 48 hours after rotation. The token can be
        rotated a maximum of three times within the 48-hour period."""
        response = await self._request_asyncio(
            method="post",
            path="/ingest/api/v1/source/{source_id}/token/rotate",
            path_params={
                "source_id": source_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return RotateTokenOut.model_validate(response.json())


class IngestSource(ApiBase):
    def list(
        self, options: IngestSourceListOptions = IngestSourceListOptions()
    ) -> ListResponseIngestSourceOut:
        """List of all the organization's Ingest Sources."""
        response = self._request_sync(
            method="get",
            path="/ingest/api/v1/source",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseIngestSourceOut.model_validate(response.json())

    def create(
        self,
        ingest_source_in: IngestSourceIn,
        options: IngestSourceCreateOptions = IngestSourceCreateOptions(),
    ) -> IngestSourceOut:
        """Create Ingest Source."""
        response = self._request_sync(
            method="post",
            path="/ingest/api/v1/source",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=ingest_source_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return IngestSourceOut.model_validate(response.json())

    def get(self, source_id: str) -> IngestSourceOut:
        """Get an Ingest Source by id or uid."""
        response = self._request_sync(
            method="get",
            path="/ingest/api/v1/source/{source_id}",
            path_params={
                "source_id": source_id,
            },
        )
        return IngestSourceOut.model_validate(response.json())

    def update(
        self, source_id: str, ingest_source_in: IngestSourceIn
    ) -> IngestSourceOut:
        """Update an Ingest Source."""
        response = self._request_sync(
            method="put",
            path="/ingest/api/v1/source/{source_id}",
            path_params={
                "source_id": source_id,
            },
            json_body=ingest_source_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return IngestSourceOut.model_validate(response.json())

    def delete(self, source_id: str) -> None:
        """Delete an Ingest Source."""
        self._request_sync(
            method="delete",
            path="/ingest/api/v1/source/{source_id}",
            path_params={
                "source_id": source_id,
            },
        )

    def rotate_token(
        self,
        source_id: str,
        options: IngestSourceRotateTokenOptions = IngestSourceRotateTokenOptions(),
    ) -> RotateTokenOut:
        """Rotate the Ingest Source's Url Token.

        This will rotate the ingest source's token, which is used to
        construct the unique `ingestUrl` for the source. Previous tokens
        will remain valid for 48 hours after rotation. The token can be
        rotated a maximum of three times within the 48-hour period."""
        response = self._request_sync(
            method="post",
            path="/ingest/api/v1/source/{source_id}/token/rotate",
            path_params={
                "source_id": source_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return RotateTokenOut.model_validate(response.json())
