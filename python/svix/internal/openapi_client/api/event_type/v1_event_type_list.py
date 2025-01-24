import asyncio
import random
from http import HTTPStatus
from time import sleep
from typing import Any, Dict, Union

import httpx

from ... import errors
from ...client import AuthenticatedClient, Client
from ...models.http_error import HttpError
from ...models.http_validation_error import HTTPValidationError
from ...models.list_response_event_type_out import ListResponseEventTypeOut
from ...models.ordering import Ordering
from ...types import UNSET, Response, Unset


def _get_kwargs(
    *,
    client: AuthenticatedClient,
    limit: Union[Unset, None, int] = UNSET,
    iterator: Union[Unset, None, str] = UNSET,
    order: Union[Unset, None, Ordering] = UNSET,
    include_archived: Union[Unset, None, bool] = False,
    with_content: Union[Unset, None, bool] = False,
) -> Dict[str, Any]:
    url = "{}/api/v1/event-type".format(client.base_url)

    headers: Dict[str, str] = client.get_headers()
    cookies: Dict[str, Any] = client.get_cookies()

    params: Dict[str, Any] = {}
    params["limit"] = limit

    params["iterator"] = iterator

    json_order: Union[Unset, None, str] = UNSET
    if not isinstance(order, Unset):
        json_order = order.value if order else None

    params["order"] = json_order

    params["include_archived"] = include_archived

    params["with_content"] = with_content

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


def _parse_response(*, client: Client, response: httpx.Response) -> ListResponseEventTypeOut:
    if response.status_code == HTTPStatus.OK:
        response_200 = ListResponseEventTypeOut.from_dict(response.json())

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


def _build_response(*, client: Client, response: httpx.Response) -> Response[ListResponseEventTypeOut]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def request_sync_detailed(
    *,
    client: AuthenticatedClient,
    limit: Union[Unset, None, int] = UNSET,
    iterator: Union[Unset, None, str] = UNSET,
    order: Union[Unset, None, Ordering] = UNSET,
    include_archived: Union[Unset, None, bool] = False,
    with_content: Union[Unset, None, bool] = False,
) -> Response[ListResponseEventTypeOut]:
    """List Event Types

     Return the list of event types.

    Args:
        limit (Union[Unset, None, int]): Limit the number of returned items
        iterator (Union[Unset, None, str]): The iterator returned from a prior invocation Example:
            user.signup.
        order (Union[Unset, None, Ordering]): Defines the ordering in a listing of results.
        include_archived (Union[Unset, None, bool]): When `true` archived (deleted but not
            expunged) items are included in the response.
        with_content (Union[Unset, None, bool]): When `true` the full item (including the schema)
            is included in the response.

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[ListResponseEventTypeOut]
    """

    kwargs = _get_kwargs(
        client=client,
        limit=limit,
        iterator=iterator,
        order=order,
        include_archived=include_archived,
        with_content=with_content,
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
    *,
    client: AuthenticatedClient,
    limit: Union[Unset, None, int] = UNSET,
    iterator: Union[Unset, None, str] = UNSET,
    order: Union[Unset, None, Ordering] = UNSET,
    include_archived: Union[Unset, None, bool] = False,
    with_content: Union[Unset, None, bool] = False,
) -> ListResponseEventTypeOut:
    """List Event Types

     Return the list of event types.

    Args:
        limit (Union[Unset, None, int]): Limit the number of returned items
        iterator (Union[Unset, None, str]): The iterator returned from a prior invocation Example:
            user.signup.
        order (Union[Unset, None, Ordering]): Defines the ordering in a listing of results.
        include_archived (Union[Unset, None, bool]): When `true` archived (deleted but not
            expunged) items are included in the response.
        with_content (Union[Unset, None, bool]): When `true` the full item (including the schema)
            is included in the response.

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        ListResponseEventTypeOut
    """

    return request_sync_detailed(
        client=client,
        limit=limit,
        iterator=iterator,
        order=order,
        include_archived=include_archived,
        with_content=with_content,
    ).parsed


async def request_asyncio_detailed(
    *,
    client: AuthenticatedClient,
    limit: Union[Unset, None, int] = UNSET,
    iterator: Union[Unset, None, str] = UNSET,
    order: Union[Unset, None, Ordering] = UNSET,
    include_archived: Union[Unset, None, bool] = False,
    with_content: Union[Unset, None, bool] = False,
) -> Response[ListResponseEventTypeOut]:
    """List Event Types

     Return the list of event types.

    Args:
        limit (Union[Unset, None, int]): Limit the number of returned items
        iterator (Union[Unset, None, str]): The iterator returned from a prior invocation Example:
            user.signup.
        order (Union[Unset, None, Ordering]): Defines the ordering in a listing of results.
        include_archived (Union[Unset, None, bool]): When `true` archived (deleted but not
            expunged) items are included in the response.
        with_content (Union[Unset, None, bool]): When `true` the full item (including the schema)
            is included in the response.

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[ListResponseEventTypeOut]
    """

    kwargs = _get_kwargs(
        client=client,
        limit=limit,
        iterator=iterator,
        order=order,
        include_archived=include_archived,
        with_content=with_content,
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
    *,
    client: AuthenticatedClient,
    limit: Union[Unset, None, int] = UNSET,
    iterator: Union[Unset, None, str] = UNSET,
    order: Union[Unset, None, Ordering] = UNSET,
    include_archived: Union[Unset, None, bool] = False,
    with_content: Union[Unset, None, bool] = False,
) -> ListResponseEventTypeOut:
    """List Event Types

     Return the list of event types.

    Args:
        limit (Union[Unset, None, int]): Limit the number of returned items
        iterator (Union[Unset, None, str]): The iterator returned from a prior invocation Example:
            user.signup.
        order (Union[Unset, None, Ordering]): Defines the ordering in a listing of results.
        include_archived (Union[Unset, None, bool]): When `true` archived (deleted but not
            expunged) items are included in the response.
        with_content (Union[Unset, None, bool]): When `true` the full item (including the schema)
            is included in the response.

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        ListResponseEventTypeOut
    """

    return (
        await request_asyncio_detailed(
            client=client,
            limit=limit,
            iterator=iterator,
            order=order,
            include_archived=include_archived,
            with_content=with_content,
        )
    ).parsed
