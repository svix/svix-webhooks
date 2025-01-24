import asyncio
import random
from http import HTTPStatus
from time import sleep
from typing import Any, Dict

import httpx

from ... import errors
from ...client import AuthenticatedClient, Client
from ...models.event_type_out import EventTypeOut
from ...models.event_type_update import EventTypeUpdate
from ...models.http_error import HttpError
from ...models.http_validation_error import HTTPValidationError
from ...types import Response


def _get_kwargs(
    event_type_name: str,
    *,
    client: AuthenticatedClient,
    json_body: EventTypeUpdate,
) -> Dict[str, Any]:
    url = "{}/api/v1/event-type/{event_type_name}".format(client.base_url, event_type_name=event_type_name)

    headers: Dict[str, str] = client.get_headers()
    cookies: Dict[str, Any] = client.get_cookies()

    json_json_body = json_body.to_dict()

    return {
        "method": "put",
        "url": url,
        "headers": headers,
        "cookies": cookies,
        "timeout": client.get_timeout(),
        "follow_redirects": client.follow_redirects,
        "json": json_json_body,
    }


def _parse_response(*, client: Client, response: httpx.Response) -> EventTypeOut:
    if response.status_code == HTTPStatus.OK:
        response_200 = EventTypeOut.from_dict(response.json())

        return response_200
    if response.status_code == HTTPStatus.CREATED:
        response_201 = EventTypeOut.from_dict(response.json())

        return response_201
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


def _build_response(*, client: Client, response: httpx.Response) -> Response[EventTypeOut]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def request_sync_detailed(
    event_type_name: str,
    *,
    client: AuthenticatedClient,
    json_body: EventTypeUpdate,
) -> Response[EventTypeOut]:
    """Update Event Type

     Update an event type.

    Args:
        event_type_name (str): The event type's name Example: user.signup.
        json_body (EventTypeUpdate):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[EventTypeOut]
    """

    kwargs = _get_kwargs(
        event_type_name=event_type_name,
        client=client,
        json_body=json_body,
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
    event_type_name: str,
    *,
    client: AuthenticatedClient,
    json_body: EventTypeUpdate,
) -> EventTypeOut:
    """Update Event Type

     Update an event type.

    Args:
        event_type_name (str): The event type's name Example: user.signup.
        json_body (EventTypeUpdate):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        EventTypeOut
    """

    return request_sync_detailed(
        event_type_name=event_type_name,
        client=client,
        json_body=json_body,
    ).parsed


async def request_asyncio_detailed(
    event_type_name: str,
    *,
    client: AuthenticatedClient,
    json_body: EventTypeUpdate,
) -> Response[EventTypeOut]:
    """Update Event Type

     Update an event type.

    Args:
        event_type_name (str): The event type's name Example: user.signup.
        json_body (EventTypeUpdate):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[EventTypeOut]
    """

    kwargs = _get_kwargs(
        event_type_name=event_type_name,
        client=client,
        json_body=json_body,
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
    event_type_name: str,
    *,
    client: AuthenticatedClient,
    json_body: EventTypeUpdate,
) -> EventTypeOut:
    """Update Event Type

     Update an event type.

    Args:
        event_type_name (str): The event type's name Example: user.signup.
        json_body (EventTypeUpdate):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        EventTypeOut
    """

    return (
        await request_asyncio_detailed(
            event_type_name=event_type_name,
            client=client,
            json_body=json_body,
        )
    ).parsed
