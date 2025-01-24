import asyncio
import datetime
import random
from http import HTTPStatus
from time import sleep
from typing import Any, Dict, Union

import httpx

from ... import errors
from ...client import AuthenticatedClient, Client
from ...models.endpoint_stats import EndpointStats
from ...models.http_error import HttpError
from ...models.http_validation_error import HTTPValidationError
from ...types import UNSET, Response, Unset


def _get_kwargs(
    app_id: str,
    endpoint_id: str,
    *,
    client: AuthenticatedClient,
    since: Union[Unset, None, datetime.datetime] = UNSET,
    until: Union[Unset, None, datetime.datetime] = UNSET,
) -> Dict[str, Any]:
    url = "{}/api/v1/app/{app_id}/endpoint/{endpoint_id}/stats".format(
        client.base_url, app_id=app_id, endpoint_id=endpoint_id
    )

    headers: Dict[str, str] = client.get_headers()
    cookies: Dict[str, Any] = client.get_cookies()

    params: Dict[str, Any] = {}
    json_since: Union[Unset, None, str] = UNSET
    if not isinstance(since, Unset):
        json_since = since.isoformat() if since else None

    params["since"] = json_since

    json_until: Union[Unset, None, str] = UNSET
    if not isinstance(until, Unset):
        json_until = until.isoformat() if until else None

    params["until"] = json_until

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


def _parse_response(*, client: Client, response: httpx.Response) -> EndpointStats:
    if response.status_code == HTTPStatus.OK:
        response_200 = EndpointStats.from_dict(response.json())

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


def _build_response(*, client: Client, response: httpx.Response) -> Response[EndpointStats]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def request_sync_detailed(
    app_id: str,
    endpoint_id: str,
    *,
    client: AuthenticatedClient,
    since: Union[Unset, None, datetime.datetime] = UNSET,
    until: Union[Unset, None, datetime.datetime] = UNSET,
) -> Response[EndpointStats]:
    """Endpoint Stats

     Get basic statistics for the endpoint.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        endpoint_id (str): The ep's ID or UID Example: unique-ep-identifier.
        since (Union[Unset, None, datetime.datetime]): Filter the range to data starting from this
            date.
        until (Union[Unset, None, datetime.datetime]): Filter the range to data ending by this
            date.

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[EndpointStats]
    """

    kwargs = _get_kwargs(
        app_id=app_id,
        endpoint_id=endpoint_id,
        client=client,
        since=since,
        until=until,
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
    endpoint_id: str,
    *,
    client: AuthenticatedClient,
    since: Union[Unset, None, datetime.datetime] = UNSET,
    until: Union[Unset, None, datetime.datetime] = UNSET,
) -> EndpointStats:
    """Endpoint Stats

     Get basic statistics for the endpoint.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        endpoint_id (str): The ep's ID or UID Example: unique-ep-identifier.
        since (Union[Unset, None, datetime.datetime]): Filter the range to data starting from this
            date.
        until (Union[Unset, None, datetime.datetime]): Filter the range to data ending by this
            date.

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        EndpointStats
    """

    return request_sync_detailed(
        app_id=app_id,
        endpoint_id=endpoint_id,
        client=client,
        since=since,
        until=until,
    ).parsed


async def request_asyncio_detailed(
    app_id: str,
    endpoint_id: str,
    *,
    client: AuthenticatedClient,
    since: Union[Unset, None, datetime.datetime] = UNSET,
    until: Union[Unset, None, datetime.datetime] = UNSET,
) -> Response[EndpointStats]:
    """Endpoint Stats

     Get basic statistics for the endpoint.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        endpoint_id (str): The ep's ID or UID Example: unique-ep-identifier.
        since (Union[Unset, None, datetime.datetime]): Filter the range to data starting from this
            date.
        until (Union[Unset, None, datetime.datetime]): Filter the range to data ending by this
            date.

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[EndpointStats]
    """

    kwargs = _get_kwargs(
        app_id=app_id,
        endpoint_id=endpoint_id,
        client=client,
        since=since,
        until=until,
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
    endpoint_id: str,
    *,
    client: AuthenticatedClient,
    since: Union[Unset, None, datetime.datetime] = UNSET,
    until: Union[Unset, None, datetime.datetime] = UNSET,
) -> EndpointStats:
    """Endpoint Stats

     Get basic statistics for the endpoint.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        endpoint_id (str): The ep's ID or UID Example: unique-ep-identifier.
        since (Union[Unset, None, datetime.datetime]): Filter the range to data starting from this
            date.
        until (Union[Unset, None, datetime.datetime]): Filter the range to data ending by this
            date.

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        EndpointStats
    """

    return (
        await request_asyncio_detailed(
            app_id=app_id,
            endpoint_id=endpoint_id,
            client=client,
            since=since,
            until=until,
        )
    ).parsed
