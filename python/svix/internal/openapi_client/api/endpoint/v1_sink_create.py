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
from ...models.sink_in_type_0 import SinkInType0
from ...models.sink_in_type_1 import SinkInType1
from ...models.sink_in_type_2 import SinkInType2
from ...models.sink_in_type_3 import SinkInType3
from ...models.sink_in_type_4 import SinkInType4
from ...models.sink_out_type_0 import SinkOutType0
from ...models.sink_out_type_1 import SinkOutType1
from ...models.sink_out_type_2 import SinkOutType2
from ...models.sink_out_type_3 import SinkOutType3
from ...models.sink_out_type_4 import SinkOutType4
from ...types import UNSET, Response, Unset


def _get_kwargs(
    app_id: str,
    *,
    client: AuthenticatedClient,
    json_body: Union["SinkInType0", "SinkInType1", "SinkInType2", "SinkInType3", "SinkInType4"],
    idempotency_key: Union[Unset, str] = UNSET,
) -> Dict[str, Any]:
    url = "{}/api/v1/app/{app_id}/sink".format(client.base_url, app_id=app_id)

    headers: Dict[str, str] = client.get_headers()
    cookies: Dict[str, Any] = client.get_cookies()

    if not isinstance(idempotency_key, Unset):
        headers["idempotency-key"] = idempotency_key

    json_json_body: Dict[str, Any]

    if isinstance(json_body, SinkInType0):
        json_json_body = json_body.to_dict()

    elif isinstance(json_body, SinkInType1):
        json_json_body = json_body.to_dict()

    elif isinstance(json_body, SinkInType2):
        json_json_body = json_body.to_dict()

    elif isinstance(json_body, SinkInType3):
        json_json_body = json_body.to_dict()

    else:
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


def _parse_response(
    *, client: Client, response: httpx.Response
) -> Union["SinkOutType0", "SinkOutType1", "SinkOutType2", "SinkOutType3", "SinkOutType4"]:
    if response.status_code == HTTPStatus.CREATED:

        def _parse_response_201(
            data: object,
        ) -> Union["SinkOutType0", "SinkOutType1", "SinkOutType2", "SinkOutType3", "SinkOutType4"]:
            try:
                if not isinstance(data, dict):
                    raise TypeError()
                componentsschemas_sink_out_type_0 = SinkOutType0.from_dict(data)

                return componentsschemas_sink_out_type_0
            except:  # noqa: E722
                pass
            try:
                if not isinstance(data, dict):
                    raise TypeError()
                componentsschemas_sink_out_type_1 = SinkOutType1.from_dict(data)

                return componentsschemas_sink_out_type_1
            except:  # noqa: E722
                pass
            try:
                if not isinstance(data, dict):
                    raise TypeError()
                componentsschemas_sink_out_type_2 = SinkOutType2.from_dict(data)

                return componentsschemas_sink_out_type_2
            except:  # noqa: E722
                pass
            try:
                if not isinstance(data, dict):
                    raise TypeError()
                componentsschemas_sink_out_type_3 = SinkOutType3.from_dict(data)

                return componentsschemas_sink_out_type_3
            except:  # noqa: E722
                pass
            if not isinstance(data, dict):
                raise TypeError()
            componentsschemas_sink_out_type_4 = SinkOutType4.from_dict(data)

            return componentsschemas_sink_out_type_4

        response_201 = _parse_response_201(response.json())

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


def _build_response(
    *, client: Client, response: httpx.Response
) -> Response[Union["SinkOutType0", "SinkOutType1", "SinkOutType2", "SinkOutType3", "SinkOutType4"]]:
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
    json_body: Union["SinkInType0", "SinkInType1", "SinkInType2", "SinkInType3", "SinkInType4"],
    idempotency_key: Union[Unset, str] = UNSET,
) -> Response[Union["SinkOutType0", "SinkOutType1", "SinkOutType2", "SinkOutType3", "SinkOutType4"]]:
    """Create Sink

     Create a new sink for the application.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        idempotency_key (Union[Unset, str]):
        json_body (Union['SinkInType0', 'SinkInType1', 'SinkInType2', 'SinkInType3',
            'SinkInType4']):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[Union['SinkOutType0', 'SinkOutType1', 'SinkOutType2', 'SinkOutType3', 'SinkOutType4']]
    """

    kwargs = _get_kwargs(
        app_id=app_id,
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
    app_id: str,
    *,
    client: AuthenticatedClient,
    json_body: Union["SinkInType0", "SinkInType1", "SinkInType2", "SinkInType3", "SinkInType4"],
    idempotency_key: Union[Unset, str] = UNSET,
) -> Union["SinkOutType0", "SinkOutType1", "SinkOutType2", "SinkOutType3", "SinkOutType4"]:
    """Create Sink

     Create a new sink for the application.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        idempotency_key (Union[Unset, str]):
        json_body (Union['SinkInType0', 'SinkInType1', 'SinkInType2', 'SinkInType3',
            'SinkInType4']):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Union['SinkOutType0', 'SinkOutType1', 'SinkOutType2', 'SinkOutType3', 'SinkOutType4']
    """

    return request_sync_detailed(
        app_id=app_id,
        client=client,
        json_body=json_body,
        idempotency_key=idempotency_key,
    ).parsed


async def request_asyncio_detailed(
    app_id: str,
    *,
    client: AuthenticatedClient,
    json_body: Union["SinkInType0", "SinkInType1", "SinkInType2", "SinkInType3", "SinkInType4"],
    idempotency_key: Union[Unset, str] = UNSET,
) -> Response[Union["SinkOutType0", "SinkOutType1", "SinkOutType2", "SinkOutType3", "SinkOutType4"]]:
    """Create Sink

     Create a new sink for the application.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        idempotency_key (Union[Unset, str]):
        json_body (Union['SinkInType0', 'SinkInType1', 'SinkInType2', 'SinkInType3',
            'SinkInType4']):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[Union['SinkOutType0', 'SinkOutType1', 'SinkOutType2', 'SinkOutType3', 'SinkOutType4']]
    """

    kwargs = _get_kwargs(
        app_id=app_id,
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
    app_id: str,
    *,
    client: AuthenticatedClient,
    json_body: Union["SinkInType0", "SinkInType1", "SinkInType2", "SinkInType3", "SinkInType4"],
    idempotency_key: Union[Unset, str] = UNSET,
) -> Union["SinkOutType0", "SinkOutType1", "SinkOutType2", "SinkOutType3", "SinkOutType4"]:
    """Create Sink

     Create a new sink for the application.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        idempotency_key (Union[Unset, str]):
        json_body (Union['SinkInType0', 'SinkInType1', 'SinkInType2', 'SinkInType3',
            'SinkInType4']):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Union['SinkOutType0', 'SinkOutType1', 'SinkOutType2', 'SinkOutType3', 'SinkOutType4']
    """

    return (
        await request_asyncio_detailed(
            app_id=app_id,
            client=client,
            json_body=json_body,
            idempotency_key=idempotency_key,
        )
    ).parsed
