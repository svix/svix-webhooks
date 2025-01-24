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
from ...models.message_in import MessageIn
from ...models.message_out import MessageOut
from ...types import UNSET, Response, Unset


def _get_kwargs(
    app_id: str,
    *,
    client: AuthenticatedClient,
    json_body: MessageIn,
    with_content: Union[Unset, None, bool] = True,
    idempotency_key: Union[Unset, str] = UNSET,
) -> Dict[str, Any]:
    url = "{}/api/v1/app/{app_id}/msg".format(client.base_url, app_id=app_id)

    headers: Dict[str, str] = client.get_headers()
    cookies: Dict[str, Any] = client.get_cookies()

    if not isinstance(idempotency_key, Unset):
        headers["idempotency-key"] = idempotency_key

    params: Dict[str, Any] = {}
    params["with_content"] = with_content

    params = {k: v for k, v in params.items() if v is not UNSET and v is not None}

    json_json_body = json_body.to_dict()

    return {
        "method": "post",
        "url": url,
        "headers": headers,
        "cookies": cookies,
        "timeout": client.get_timeout(),
        "follow_redirects": client.follow_redirects,
        "json": json_json_body,
        "params": params,
    }


def _parse_response(*, client: Client, response: httpx.Response) -> MessageOut:
    if response.status_code == HTTPStatus.ACCEPTED:
        response_202 = MessageOut.from_dict(response.json())

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
    if response.status_code == HTTPStatus.REQUEST_ENTITY_TOO_LARGE:
        raise HttpError.init_exception(response.json(), response.status_code)
    if response.status_code == HTTPStatus.UNPROCESSABLE_ENTITY:
        raise HTTPValidationError.init_exception(response.json(), response.status_code)
    if response.status_code == HTTPStatus.TOO_MANY_REQUESTS:
        raise HttpError.init_exception(response.json(), response.status_code)
    raise errors.UnexpectedStatus(response.status_code, response.content)


def _build_response(*, client: Client, response: httpx.Response) -> Response[MessageOut]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def request_sync_detailed(
    app_id: str,
    *,
    client: AuthenticatedClient,
    json_body: MessageIn,
    with_content: Union[Unset, None, bool] = True,
    idempotency_key: Union[Unset, str] = UNSET,
) -> Response[MessageOut]:
    """Create Message

     Creates a new message and dispatches it to all of the application's endpoints.

    The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day, after
    that no verification will be made.
    If a message with the same `eventId` already exists for the application, a 409 conflict error will
    be returned.

    The `eventType` indicates the type and schema of the event. All messages of a certain `eventType`
    are expected to have the same schema. Endpoints can choose to only listen to specific event types.
    Messages can also have `channels`, which similar to event types let endpoints filter by them. Unlike
    event types, messages can have multiple channels, and channels don't imply a specific message
    content or schema.

    The `payload` property is the webhook's body (the actual webhook message). Svix supports payload
    sizes of up to ~350kb, though it's generally a good idea to keep webhook payloads small, probably no
    larger than 40kb.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        with_content (Union[Unset, None, bool]): When `true`, message payloads are included in the
            response. Default: True.
        idempotency_key (Union[Unset, str]):
        json_body (MessageIn):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[MessageOut]
    """

    kwargs = _get_kwargs(
        app_id=app_id,
        client=client,
        json_body=json_body,
        with_content=with_content,
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
    app_id: str,
    *,
    client: AuthenticatedClient,
    json_body: MessageIn,
    with_content: Union[Unset, None, bool] = True,
    idempotency_key: Union[Unset, str] = UNSET,
) -> MessageOut:
    """Create Message

     Creates a new message and dispatches it to all of the application's endpoints.

    The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day, after
    that no verification will be made.
    If a message with the same `eventId` already exists for the application, a 409 conflict error will
    be returned.

    The `eventType` indicates the type and schema of the event. All messages of a certain `eventType`
    are expected to have the same schema. Endpoints can choose to only listen to specific event types.
    Messages can also have `channels`, which similar to event types let endpoints filter by them. Unlike
    event types, messages can have multiple channels, and channels don't imply a specific message
    content or schema.

    The `payload` property is the webhook's body (the actual webhook message). Svix supports payload
    sizes of up to ~350kb, though it's generally a good idea to keep webhook payloads small, probably no
    larger than 40kb.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        with_content (Union[Unset, None, bool]): When `true`, message payloads are included in the
            response. Default: True.
        idempotency_key (Union[Unset, str]):
        json_body (MessageIn):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        MessageOut
    """

    return request_sync_detailed(
        app_id=app_id,
        client=client,
        json_body=json_body,
        with_content=with_content,
        idempotency_key=idempotency_key,
    ).parsed


async def request_asyncio_detailed(
    app_id: str,
    *,
    client: AuthenticatedClient,
    json_body: MessageIn,
    with_content: Union[Unset, None, bool] = True,
    idempotency_key: Union[Unset, str] = UNSET,
) -> Response[MessageOut]:
    """Create Message

     Creates a new message and dispatches it to all of the application's endpoints.

    The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day, after
    that no verification will be made.
    If a message with the same `eventId` already exists for the application, a 409 conflict error will
    be returned.

    The `eventType` indicates the type and schema of the event. All messages of a certain `eventType`
    are expected to have the same schema. Endpoints can choose to only listen to specific event types.
    Messages can also have `channels`, which similar to event types let endpoints filter by them. Unlike
    event types, messages can have multiple channels, and channels don't imply a specific message
    content or schema.

    The `payload` property is the webhook's body (the actual webhook message). Svix supports payload
    sizes of up to ~350kb, though it's generally a good idea to keep webhook payloads small, probably no
    larger than 40kb.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        with_content (Union[Unset, None, bool]): When `true`, message payloads are included in the
            response. Default: True.
        idempotency_key (Union[Unset, str]):
        json_body (MessageIn):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[MessageOut]
    """

    kwargs = _get_kwargs(
        app_id=app_id,
        client=client,
        json_body=json_body,
        with_content=with_content,
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
    app_id: str,
    *,
    client: AuthenticatedClient,
    json_body: MessageIn,
    with_content: Union[Unset, None, bool] = True,
    idempotency_key: Union[Unset, str] = UNSET,
) -> MessageOut:
    """Create Message

     Creates a new message and dispatches it to all of the application's endpoints.

    The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day, after
    that no verification will be made.
    If a message with the same `eventId` already exists for the application, a 409 conflict error will
    be returned.

    The `eventType` indicates the type and schema of the event. All messages of a certain `eventType`
    are expected to have the same schema. Endpoints can choose to only listen to specific event types.
    Messages can also have `channels`, which similar to event types let endpoints filter by them. Unlike
    event types, messages can have multiple channels, and channels don't imply a specific message
    content or schema.

    The `payload` property is the webhook's body (the actual webhook message). Svix supports payload
    sizes of up to ~350kb, though it's generally a good idea to keep webhook payloads small, probably no
    larger than 40kb.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        with_content (Union[Unset, None, bool]): When `true`, message payloads are included in the
            response. Default: True.
        idempotency_key (Union[Unset, str]):
        json_body (MessageIn):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        MessageOut
    """

    return (
        await request_asyncio_detailed(
            app_id=app_id,
            client=client,
            json_body=json_body,
            with_content=with_content,
            idempotency_key=idempotency_key,
        )
    ).parsed
