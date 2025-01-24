import asyncio
import random
from http import HTTPStatus
from time import sleep
from typing import Any, Dict

import httpx

from ... import errors
from ...client import AuthenticatedClient, Client
from ...models.http_error import HttpError
from ...models.http_validation_error import HTTPValidationError
from ...models.message_attempt_out import MessageAttemptOut
from ...types import Response


def _get_kwargs(
    app_id: str,
    msg_id: str,
    attempt_id: str,
    *,
    client: AuthenticatedClient,
) -> Dict[str, Any]:
    url = "{}/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}".format(
        client.base_url, app_id=app_id, msg_id=msg_id, attempt_id=attempt_id
    )

    headers: Dict[str, str] = client.get_headers()
    cookies: Dict[str, Any] = client.get_cookies()

    return {
        "method": "get",
        "url": url,
        "headers": headers,
        "cookies": cookies,
        "timeout": client.get_timeout(),
        "follow_redirects": client.follow_redirects,
    }


def _parse_response(*, client: Client, response: httpx.Response) -> MessageAttemptOut:
    if response.status_code == HTTPStatus.OK:
        response_200 = MessageAttemptOut.from_dict(response.json())

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


def _build_response(*, client: Client, response: httpx.Response) -> Response[MessageAttemptOut]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def request_sync_detailed(
    app_id: str,
    msg_id: str,
    attempt_id: str,
    *,
    client: AuthenticatedClient,
) -> Response[MessageAttemptOut]:
    """Get Attempt

     `msg_id`: Use a message id or a message `eventId`

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        msg_id (str): The msg's ID or UID Example: unique-msg-identifier.
        attempt_id (str): The attempt's ID Example: atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2.

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[MessageAttemptOut]
    """

    kwargs = _get_kwargs(
        app_id=app_id,
        msg_id=msg_id,
        attempt_id=attempt_id,
        client=client,
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
    msg_id: str,
    attempt_id: str,
    *,
    client: AuthenticatedClient,
) -> MessageAttemptOut:
    """Get Attempt

     `msg_id`: Use a message id or a message `eventId`

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        msg_id (str): The msg's ID or UID Example: unique-msg-identifier.
        attempt_id (str): The attempt's ID Example: atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2.

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        MessageAttemptOut
    """

    return request_sync_detailed(
        app_id=app_id,
        msg_id=msg_id,
        attempt_id=attempt_id,
        client=client,
    ).parsed


async def request_asyncio_detailed(
    app_id: str,
    msg_id: str,
    attempt_id: str,
    *,
    client: AuthenticatedClient,
) -> Response[MessageAttemptOut]:
    """Get Attempt

     `msg_id`: Use a message id or a message `eventId`

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        msg_id (str): The msg's ID or UID Example: unique-msg-identifier.
        attempt_id (str): The attempt's ID Example: atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2.

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[MessageAttemptOut]
    """

    kwargs = _get_kwargs(
        app_id=app_id,
        msg_id=msg_id,
        attempt_id=attempt_id,
        client=client,
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
    msg_id: str,
    attempt_id: str,
    *,
    client: AuthenticatedClient,
) -> MessageAttemptOut:
    """Get Attempt

     `msg_id`: Use a message id or a message `eventId`

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        msg_id (str): The msg's ID or UID Example: unique-msg-identifier.
        attempt_id (str): The attempt's ID Example: atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2.

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        MessageAttemptOut
    """

    return (
        await request_asyncio_detailed(
            app_id=app_id,
            msg_id=msg_id,
            attempt_id=attempt_id,
            client=client,
        )
    ).parsed
