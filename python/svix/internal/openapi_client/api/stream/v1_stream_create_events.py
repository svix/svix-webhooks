import asyncio
import random
from http import HTTPStatus
from time import sleep
from typing import Any, Dict, Union

import httpx

from ... import errors
from ...client import AuthenticatedClient, Client
from ...models.create_stream_in import CreateStreamIn
from ...models.create_stream_out import CreateStreamOut
from ...models.http_error import HttpError
from ...models.http_validation_error import HTTPValidationError
from ...types import UNSET, Response, Unset


def _get_kwargs(
    stream_id: str,
    *,
    client: AuthenticatedClient,
    json_body: CreateStreamIn,
    idempotency_key: Union[Unset, str] = UNSET,
) -> Dict[str, Any]:
    url = "{}/api/v1/stream/{stream_id}/events".format(client.base_url, stream_id=stream_id)

    headers: Dict[str, str] = client.get_headers()
    cookies: Dict[str, Any] = client.get_cookies()

    if not isinstance(idempotency_key, Unset):
        headers["idempotency-key"] = idempotency_key

    json_json_body = json_body.to_dict()

    return {
        "method": "post",
        "url": url,
        "headers": headers,
        "cookies": cookies,
        "timeout": client.get_timeout(),
        "follow_redirects": client.follow_redirects,
        "json": json_json_body,
    }


def _parse_response(*, client: Client, response: httpx.Response) -> CreateStreamOut:
    if response.status_code == HTTPStatus.ACCEPTED:
        response_202 = CreateStreamOut.from_dict(response.json())

        return response_202
    if response.status_code == HTTPStatus.BAD_REQUEST:
        raise HttpError.init_exception(response.json(), response.status_code)
    if response.status_code == HTTPStatus.UNAUTHORIZED:
        raise HttpError.init_exception(response.json(), response.status_code)
    if response.status_code == HTTPStatus.FORBIDDEN:
        raise HttpError.init_exception(response.json(), response.status_code)
    if response.status_code == HTTPStatus.NOT_FOUND:
        raise HttpError.init_exception(response.json(), response.status_code)
    if response.status_code == HTTPStatus.CONFLICT:
        raise HttpError.init_exception(response.json(), response.status_code)
    if response.status_code == HTTPStatus.UNPROCESSABLE_ENTITY:
        raise HTTPValidationError.init_exception(response.json(), response.status_code)
    if response.status_code == HTTPStatus.TOO_MANY_REQUESTS:
        raise HttpError.init_exception(response.json(), response.status_code)
    raise errors.UnexpectedStatus(response.status_code, response.content)


def _build_response(*, client: Client, response: httpx.Response) -> Response[CreateStreamOut]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def request_sync_detailed(
    stream_id: str,
    *,
    client: AuthenticatedClient,
    json_body: CreateStreamIn,
    idempotency_key: Union[Unset, str] = UNSET,
) -> Response[CreateStreamOut]:
    """Create Events

     Creates events on the Stream.

    Args:
        stream_id (str):
        idempotency_key (Union[Unset, str]):
        json_body (CreateStreamIn):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[CreateStreamOut]
    """

    kwargs = _get_kwargs(
        stream_id=stream_id,
        client=client,
        json_body=json_body,
        idempotency_key=idempotency_key,
    )

    kwargs["headers"] = {"svix-req-id": f"{random.getrandbits(32)}", **kwargs["headers"]}

    response = httpx.request(
        verify=client.verify_ssl,
        **kwargs,
    )
    for retry_count, sleep_time in enumerate(client.retry_schedule):
        if response.status_code < 500:
            break

        sleep(sleep_time)
        kwargs["headers"]["svix-retry-count"] = str(retry_count)
        response = httpx.request(
            verify=client.verify_ssl,
            **kwargs,
        )

    return _build_response(client=client, response=response)


def request_sync(
    stream_id: str,
    *,
    client: AuthenticatedClient,
    json_body: CreateStreamIn,
    idempotency_key: Union[Unset, str] = UNSET,
) -> CreateStreamOut:
    """Create Events

     Creates events on the Stream.

    Args:
        stream_id (str):
        idempotency_key (Union[Unset, str]):
        json_body (CreateStreamIn):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        CreateStreamOut
    """

    return request_sync_detailed(
        stream_id=stream_id,
        client=client,
        json_body=json_body,
        idempotency_key=idempotency_key,
    ).parsed


async def request_asyncio_detailed(
    stream_id: str,
    *,
    client: AuthenticatedClient,
    json_body: CreateStreamIn,
    idempotency_key: Union[Unset, str] = UNSET,
) -> Response[CreateStreamOut]:
    """Create Events

     Creates events on the Stream.

    Args:
        stream_id (str):
        idempotency_key (Union[Unset, str]):
        json_body (CreateStreamIn):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[CreateStreamOut]
    """

    kwargs = _get_kwargs(
        stream_id=stream_id,
        client=client,
        json_body=json_body,
        idempotency_key=idempotency_key,
    )

    kwargs["headers"] = {"svix-req-id": f"{random.getrandbits(32)}", **kwargs["headers"]}

    async with httpx.AsyncClient(verify=client.verify_ssl) as _client:
        response = await _client.request(**kwargs)

        for retry_count, sleep_time in enumerate(client.retry_schedule):
            if response.status_code < 500:
                break

            await asyncio.sleep(sleep_time)
            kwargs["headers"]["svix-retry-count"] = str(retry_count)
            response = await _client.request(**kwargs)

    return _build_response(client=client, response=response)


async def request_asyncio(
    stream_id: str,
    *,
    client: AuthenticatedClient,
    json_body: CreateStreamIn,
    idempotency_key: Union[Unset, str] = UNSET,
) -> CreateStreamOut:
    """Create Events

     Creates events on the Stream.

    Args:
        stream_id (str):
        idempotency_key (Union[Unset, str]):
        json_body (CreateStreamIn):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        CreateStreamOut
    """

    return (
        await request_asyncio_detailed(
            stream_id=stream_id,
            client=client,
            json_body=json_body,
            idempotency_key=idempotency_key,
        )
    ).parsed
