import asyncio
import random
from http import HTTPStatus
from time import sleep
from typing import Any, Dict, Union

import httpx

from ... import errors
from ...client import AuthenticatedClient, Client
from ...models.application_in import ApplicationIn
from ...models.application_out import ApplicationOut
from ...models.http_error import HttpError
from ...models.http_validation_error import HTTPValidationError
from ...types import UNSET, Response, Unset


def _get_kwargs(
    *,
    client: AuthenticatedClient,
    json_body: ApplicationIn,
    get_if_exists: Union[Unset, None, bool] = False,
    idempotency_key: Union[Unset, str] = UNSET,
) -> Dict[str, Any]:
    url = "{}/api/v1/app".format(client.base_url)

    headers: Dict[str, str] = client.get_headers()
    cookies: Dict[str, Any] = client.get_cookies()

    if not isinstance(idempotency_key, Unset):
        headers["idempotency-key"] = idempotency_key

    params: Dict[str, Any] = {}
    params["get_if_exists"] = get_if_exists

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


def _parse_response(*, client: Client, response: httpx.Response) -> ApplicationOut:
    if response.status_code == HTTPStatus.OK:
        response_200 = ApplicationOut.from_dict(response.json())

        return response_200
    if response.status_code == HTTPStatus.CREATED:
        response_201 = ApplicationOut.from_dict(response.json())

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


def _build_response(*, client: Client, response: httpx.Response) -> Response[ApplicationOut]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def request_sync_detailed(
    *,
    client: AuthenticatedClient,
    json_body: ApplicationIn,
    get_if_exists: Union[Unset, None, bool] = False,
    idempotency_key: Union[Unset, str] = UNSET,
) -> Response[ApplicationOut]:
    """Create Application

     Create a new application.

    Args:
        get_if_exists (Union[Unset, None, bool]): Get an existing application, or create a new one
            if doesn't exist. It's two separate functions in the libs.
        idempotency_key (Union[Unset, str]):
        json_body (ApplicationIn):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[ApplicationOut]
    """

    kwargs = _get_kwargs(
        client=client,
        json_body=json_body,
        get_if_exists=get_if_exists,
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
    *,
    client: AuthenticatedClient,
    json_body: ApplicationIn,
    get_if_exists: Union[Unset, None, bool] = False,
    idempotency_key: Union[Unset, str] = UNSET,
) -> ApplicationOut:
    """Create Application

     Create a new application.

    Args:
        get_if_exists (Union[Unset, None, bool]): Get an existing application, or create a new one
            if doesn't exist. It's two separate functions in the libs.
        idempotency_key (Union[Unset, str]):
        json_body (ApplicationIn):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        ApplicationOut
    """

    return request_sync_detailed(
        client=client,
        json_body=json_body,
        get_if_exists=get_if_exists,
        idempotency_key=idempotency_key,
    ).parsed


async def request_asyncio_detailed(
    *,
    client: AuthenticatedClient,
    json_body: ApplicationIn,
    get_if_exists: Union[Unset, None, bool] = False,
    idempotency_key: Union[Unset, str] = UNSET,
) -> Response[ApplicationOut]:
    """Create Application

     Create a new application.

    Args:
        get_if_exists (Union[Unset, None, bool]): Get an existing application, or create a new one
            if doesn't exist. It's two separate functions in the libs.
        idempotency_key (Union[Unset, str]):
        json_body (ApplicationIn):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[ApplicationOut]
    """

    kwargs = _get_kwargs(
        client=client,
        json_body=json_body,
        get_if_exists=get_if_exists,
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
    *,
    client: AuthenticatedClient,
    json_body: ApplicationIn,
    get_if_exists: Union[Unset, None, bool] = False,
    idempotency_key: Union[Unset, str] = UNSET,
) -> ApplicationOut:
    """Create Application

     Create a new application.

    Args:
        get_if_exists (Union[Unset, None, bool]): Get an existing application, or create a new one
            if doesn't exist. It's two separate functions in the libs.
        idempotency_key (Union[Unset, str]):
        json_body (ApplicationIn):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        ApplicationOut
    """

    return (
        await request_asyncio_detailed(
            client=client,
            json_body=json_body,
            get_if_exists=get_if_exists,
            idempotency_key=idempotency_key,
        )
    ).parsed
