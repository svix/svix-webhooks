import asyncio
import datetime
import random
from http import HTTPStatus
from time import sleep
from typing import Any, Dict, List, Union

import httpx

from ... import errors
from ...client import AuthenticatedClient, Client
from ...models.http_error import HttpError
from ...models.http_validation_error import HTTPValidationError
from ...models.message_events_out import MessageEventsOut
from ...types import UNSET, Response, Unset


def _get_kwargs(
    app_id: str,
    subscription_id: str,
    *,
    client: AuthenticatedClient,
    limit: Union[Unset, None, int] = UNSET,
    iterator: Union[Unset, None, str] = UNSET,
    event_types: Union[Unset, None, List[str]] = UNSET,
    channels: Union[Unset, None, List[str]] = UNSET,
    after: Union[Unset, None, datetime.datetime] = UNSET,
) -> Dict[str, Any]:
    url = "{}/api/v1/app/{app_id}/events/subscription/{subscription_id}".format(
        client.base_url, app_id=app_id, subscription_id=subscription_id
    )

    headers: Dict[str, str] = client.get_headers()
    cookies: Dict[str, Any] = client.get_cookies()

    params: Dict[str, Any] = {}
    params["limit"] = limit

    params["iterator"] = iterator

    json_event_types: Union[Unset, None, List[str]] = UNSET
    if not isinstance(event_types, Unset):
        if event_types is None:
            json_event_types = None
        else:
            json_event_types = event_types

    params["event_types"] = json_event_types

    json_channels: Union[Unset, None, List[str]] = UNSET
    if not isinstance(channels, Unset):
        if channels is None:
            json_channels = None
        else:
            json_channels = channels

    params["channels"] = json_channels

    json_after: Union[Unset, None, str] = UNSET
    if not isinstance(after, Unset):
        json_after = after.isoformat() if after else None

    params["after"] = json_after

    params = {k: v for k, v in params.items() if v is not UNSET and v is not None}

    return {
        "method": "get",
        "url": url,
        "headers": headers,
        "cookies": cookies,
        "timeout": client.get_timeout(),
        "follow_redirects": client.follow_redirects,
        "params": params,
    }


def _parse_response(*, client: Client, response: httpx.Response) -> MessageEventsOut:
    if response.status_code == HTTPStatus.OK:
        response_200 = MessageEventsOut.from_dict(response.json())

        return response_200
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


def _build_response(*, client: Client, response: httpx.Response) -> Response[MessageEventsOut]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def request_sync_detailed(
    app_id: str,
    subscription_id: str,
    *,
    client: AuthenticatedClient,
    limit: Union[Unset, None, int] = UNSET,
    iterator: Union[Unset, None, str] = UNSET,
    event_types: Union[Unset, None, List[str]] = UNSET,
    channels: Union[Unset, None, List[str]] = UNSET,
    after: Union[Unset, None, datetime.datetime] = UNSET,
) -> Response[MessageEventsOut]:
    """Message Events Subscription

     Reads the stream of created messages for an application, but using server-managed iterator tracking.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        subscription_id (str): The esub's ID or UID Example: unique-esub-identifier.
        limit (Union[Unset, None, int]): Limit the number of returned items
        iterator (Union[Unset, None, str]): The iterator returned from a prior invocation
        event_types (Union[Unset, None, List[str]]): Filter response based on the event type
        channels (Union[Unset, None, List[str]]): Filter response based on the event type.
        after (Union[Unset, None, datetime.datetime]):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[MessageEventsOut]
    """

    kwargs = _get_kwargs(
        app_id=app_id,
        subscription_id=subscription_id,
        client=client,
        limit=limit,
        iterator=iterator,
        event_types=event_types,
        channels=channels,
        after=after,
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
    app_id: str,
    subscription_id: str,
    *,
    client: AuthenticatedClient,
    limit: Union[Unset, None, int] = UNSET,
    iterator: Union[Unset, None, str] = UNSET,
    event_types: Union[Unset, None, List[str]] = UNSET,
    channels: Union[Unset, None, List[str]] = UNSET,
    after: Union[Unset, None, datetime.datetime] = UNSET,
) -> MessageEventsOut:
    """Message Events Subscription

     Reads the stream of created messages for an application, but using server-managed iterator tracking.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        subscription_id (str): The esub's ID or UID Example: unique-esub-identifier.
        limit (Union[Unset, None, int]): Limit the number of returned items
        iterator (Union[Unset, None, str]): The iterator returned from a prior invocation
        event_types (Union[Unset, None, List[str]]): Filter response based on the event type
        channels (Union[Unset, None, List[str]]): Filter response based on the event type.
        after (Union[Unset, None, datetime.datetime]):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        MessageEventsOut
    """

    return request_sync_detailed(
        app_id=app_id,
        subscription_id=subscription_id,
        client=client,
        limit=limit,
        iterator=iterator,
        event_types=event_types,
        channels=channels,
        after=after,
    ).parsed


async def request_asyncio_detailed(
    app_id: str,
    subscription_id: str,
    *,
    client: AuthenticatedClient,
    limit: Union[Unset, None, int] = UNSET,
    iterator: Union[Unset, None, str] = UNSET,
    event_types: Union[Unset, None, List[str]] = UNSET,
    channels: Union[Unset, None, List[str]] = UNSET,
    after: Union[Unset, None, datetime.datetime] = UNSET,
) -> Response[MessageEventsOut]:
    """Message Events Subscription

     Reads the stream of created messages for an application, but using server-managed iterator tracking.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        subscription_id (str): The esub's ID or UID Example: unique-esub-identifier.
        limit (Union[Unset, None, int]): Limit the number of returned items
        iterator (Union[Unset, None, str]): The iterator returned from a prior invocation
        event_types (Union[Unset, None, List[str]]): Filter response based on the event type
        channels (Union[Unset, None, List[str]]): Filter response based on the event type.
        after (Union[Unset, None, datetime.datetime]):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[MessageEventsOut]
    """

    kwargs = _get_kwargs(
        app_id=app_id,
        subscription_id=subscription_id,
        client=client,
        limit=limit,
        iterator=iterator,
        event_types=event_types,
        channels=channels,
        after=after,
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
    app_id: str,
    subscription_id: str,
    *,
    client: AuthenticatedClient,
    limit: Union[Unset, None, int] = UNSET,
    iterator: Union[Unset, None, str] = UNSET,
    event_types: Union[Unset, None, List[str]] = UNSET,
    channels: Union[Unset, None, List[str]] = UNSET,
    after: Union[Unset, None, datetime.datetime] = UNSET,
) -> MessageEventsOut:
    """Message Events Subscription

     Reads the stream of created messages for an application, but using server-managed iterator tracking.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        subscription_id (str): The esub's ID or UID Example: unique-esub-identifier.
        limit (Union[Unset, None, int]): Limit the number of returned items
        iterator (Union[Unset, None, str]): The iterator returned from a prior invocation
        event_types (Union[Unset, None, List[str]]): Filter response based on the event type
        channels (Union[Unset, None, List[str]]): Filter response based on the event type.
        after (Union[Unset, None, datetime.datetime]):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        MessageEventsOut
    """

    return (
        await request_asyncio_detailed(
            app_id=app_id,
            subscription_id=subscription_id,
            client=client,
            limit=limit,
            iterator=iterator,
            event_types=event_types,
            channels=channels,
            after=after,
        )
    ).parsed
