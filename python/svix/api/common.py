import asyncio
import random
import time
import typing as t
from dataclasses import asdict, dataclass
from datetime import datetime, timezone

import httpx

from .client import AuthenticatedClient
from .errors.http_error import HttpError
from .errors.http_validation_error import HTTPValidationError


def ensure_tz(x: t.Optional[datetime]) -> t.Optional[datetime]:
    if x is None:
        return None

    if x.tzinfo is None:
        return x.replace(tzinfo=timezone.utc)
    return x


def sanitize_field(v: t.Any) -> t.Any:
    if isinstance(v, datetime):
        return ensure_tz(v)

    return v


def _serialize_single_param(val: t.Any) -> str:
    if isinstance(val, datetime):
        if val.tzinfo is None:
            val.replace(tzinfo=timezone.utc)
        return val.isoformat()
    elif isinstance(val, bool):
        return "true" if val else "false"
    elif isinstance(val, set) or isinstance(val, list):
        return ",".join(val)
    else:
        return str(val)


def serialize_params(d: t.Dict[str, t.Optional[t.Any]]) -> t.Dict[str, str]:
    return {k: _serialize_single_param(v) for k, v in d.items() if v is not None}


@dataclass
class BaseOptions:
    def to_dict(self) -> t.Dict[str, t.Any]:
        return {k: sanitize_field(v) for k, v in asdict(self).items() if v is not None}

    def _query_params(self) -> t.Dict[str, str]:
        return {}

    def _header_params(self) -> t.Dict[str, str]:
        return {}


@dataclass
class ListOptions(BaseOptions):
    iterator: t.Optional[str] = None
    limit: t.Optional[int] = None


@dataclass
class PostOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


class ApiBase:
    _client: AuthenticatedClient
    _httpx_client: httpx.Client
    _httpx_async_client: httpx.AsyncClient

    def __init__(self, client: AuthenticatedClient) -> None:
        self._client = client
        self._httpx_client = httpx.Client(
            verify=client.verify_ssl, cookies=self._client.get_cookies()
        )
        self._httpx_async_client = httpx.AsyncClient(
            verify=client.verify_ssl, cookies=self._client.get_cookies()
        )

    def _get_httpx_kwargs(
        self,
        method: str,
        path: str,
        path_params: t.Optional[t.Dict[str, str]],
        query_params: t.Optional[t.Dict[str, str]],
        header_params: t.Optional[t.Dict[str, str]],
        json_body: t.Optional[str],
    ) -> t.Dict[str, t.Any]:
        if path_params is not None:
            path = path.format(**path_params)
        url = f"{self._client.base_url}{path}"

        headers: t.Dict[str, str] = {
            **self._client.get_headers(),
            "svix-req-id": f"{random.getrandbits(64)}",
        }
        if header_params is not None:
            headers.update(header_params)

        httpx_kwargs = {
            "method": method.upper(),
            "url": url,
            "headers": headers,
            "timeout": self._client.get_timeout(),
            "follow_redirects": self._client.follow_redirects,
        }

        if query_params is not None:
            httpx_kwargs["params"] = query_params

        if json_body is not None:
            encoded_body = json_body.encode("utf-8")
            httpx_kwargs["content"] = encoded_body
            headers["content-type"] = "application/json"
            headers["content-length"] = str(len(encoded_body))

        return httpx_kwargs

    async def _request_asyncio(
        self,
        method: str,
        path: str,
        path_params: t.Optional[t.Dict[str, str]] = None,
        query_params: t.Optional[t.Dict[str, str]] = None,
        header_params: t.Optional[t.Dict[str, str]] = None,
        json_body: t.Optional[str] = None,
    ) -> httpx.Response:
        httpx_kwargs = self._get_httpx_kwargs(
            method,
            path,
            path_params=path_params,
            query_params=query_params,
            header_params=header_params,
            json_body=json_body,
        )

        response = await self._httpx_async_client.request(**httpx_kwargs)

        for retry_count, sleep_time in enumerate(self._client.retry_schedule):
            if response.status_code < 500:
                break

            await asyncio.sleep(sleep_time)
            httpx_kwargs["headers"]["svix-retry-count"] = str(retry_count)
            response = await self._httpx_async_client.request(**httpx_kwargs)

        return _filter_response_for_errors_response(response)

    def _request_sync(
        self,
        method: str,
        path: str,
        path_params: t.Optional[t.Dict[str, str]] = None,
        query_params: t.Optional[t.Dict[str, str]] = None,
        header_params: t.Optional[t.Dict[str, str]] = None,
        json_body: t.Optional[str] = None,
    ) -> httpx.Response:
        httpx_kwargs = self._get_httpx_kwargs(
            method,
            path,
            path_params=path_params,
            query_params=query_params,
            header_params=header_params,
            json_body=json_body,
        )

        response = self._httpx_client.request(**httpx_kwargs)
        for retry_count, sleep_time in enumerate(self._client.retry_schedule):
            if response.status_code < 500:
                break

            time.sleep(sleep_time)
            httpx_kwargs["headers"]["svix-retry-count"] = str(retry_count)
            response = self._httpx_client.request(**httpx_kwargs)

        return _filter_response_for_errors_response(response)


def _filter_response_for_errors_response(response: httpx.Response) -> httpx.Response:
    if 200 <= response.status_code <= 299:
        return response
    else:
        if response.status_code == 422:
            raise HTTPValidationError.init_exception(
                response.json(), response.status_code
            )
        else:
            raise HttpError.init_exception(response.json(), response.status_code)
