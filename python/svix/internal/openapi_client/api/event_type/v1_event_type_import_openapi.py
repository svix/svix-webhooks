import asyncio
import random
from http import HTTPStatus
from time import sleep
from typing import Any, Dict, Union

import httpx

from ... import errors
from ...client import AuthenticatedClient, Client
from ...models.event_type_import_open_api_in import EventTypeImportOpenApiIn
from ...models.event_type_import_open_api_out import EventTypeImportOpenApiOut
from ...models.http_error import HttpError
from ...models.http_validation_error import HTTPValidationError
from ...types import UNSET, Response, Unset


def _get_kwargs(
    *,
    client: AuthenticatedClient,
    json_body: EventTypeImportOpenApiIn,
    idempotency_key: Union[Unset, str] = UNSET,
) -> Dict[str, Any]:
    url = "{}/api/v1/event-type/import/openapi".format(client.base_url)

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


def _parse_response(*, client: Client, response: httpx.Response) -> EventTypeImportOpenApiOut:
    if response.status_code == HTTPStatus.OK:
        response_200 = EventTypeImportOpenApiOut.from_dict(response.json())

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


def _build_response(*, client: Client, response: httpx.Response) -> Response[EventTypeImportOpenApiOut]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def request_sync_detailed(
    *,
    client: AuthenticatedClient,
    json_body: EventTypeImportOpenApiIn,
    idempotency_key: Union[Unset, str] = UNSET,
) -> Response[EventTypeImportOpenApiOut]:
    """Event Type Import From Openapi

     Given an OpenAPI spec, create new or update existing event types.
    If an existing `archived` event type is updated, it will be unarchived.

    The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
    top-level.

    Args:
        idempotency_key (Union[Unset, str]):
        json_body (EventTypeImportOpenApiIn): Import a list of event types from webhooks defined
            in an OpenAPI spec.

            The OpenAPI spec can be specified as either `spec` given the spec as a JSON object, or as
            `specRaw` (a `string`) which will be parsed as YAML or JSON by the server. Sending neither
            or both is invalid, resulting in a `400` **Bad Request**.

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[EventTypeImportOpenApiOut]
    """

    kwargs = _get_kwargs(
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
    *,
    client: AuthenticatedClient,
    json_body: EventTypeImportOpenApiIn,
    idempotency_key: Union[Unset, str] = UNSET,
) -> EventTypeImportOpenApiOut:
    """Event Type Import From Openapi

     Given an OpenAPI spec, create new or update existing event types.
    If an existing `archived` event type is updated, it will be unarchived.

    The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
    top-level.

    Args:
        idempotency_key (Union[Unset, str]):
        json_body (EventTypeImportOpenApiIn): Import a list of event types from webhooks defined
            in an OpenAPI spec.

            The OpenAPI spec can be specified as either `spec` given the spec as a JSON object, or as
            `specRaw` (a `string`) which will be parsed as YAML or JSON by the server. Sending neither
            or both is invalid, resulting in a `400` **Bad Request**.

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        EventTypeImportOpenApiOut
    """

    return request_sync_detailed(
        client=client,
        json_body=json_body,
        idempotency_key=idempotency_key,
    ).parsed


async def request_asyncio_detailed(
    *,
    client: AuthenticatedClient,
    json_body: EventTypeImportOpenApiIn,
    idempotency_key: Union[Unset, str] = UNSET,
) -> Response[EventTypeImportOpenApiOut]:
    """Event Type Import From Openapi

     Given an OpenAPI spec, create new or update existing event types.
    If an existing `archived` event type is updated, it will be unarchived.

    The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
    top-level.

    Args:
        idempotency_key (Union[Unset, str]):
        json_body (EventTypeImportOpenApiIn): Import a list of event types from webhooks defined
            in an OpenAPI spec.

            The OpenAPI spec can be specified as either `spec` given the spec as a JSON object, or as
            `specRaw` (a `string`) which will be parsed as YAML or JSON by the server. Sending neither
            or both is invalid, resulting in a `400` **Bad Request**.

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[EventTypeImportOpenApiOut]
    """

    kwargs = _get_kwargs(
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
    *,
    client: AuthenticatedClient,
    json_body: EventTypeImportOpenApiIn,
    idempotency_key: Union[Unset, str] = UNSET,
) -> EventTypeImportOpenApiOut:
    """Event Type Import From Openapi

     Given an OpenAPI spec, create new or update existing event types.
    If an existing `archived` event type is updated, it will be unarchived.

    The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
    top-level.

    Args:
        idempotency_key (Union[Unset, str]):
        json_body (EventTypeImportOpenApiIn): Import a list of event types from webhooks defined
            in an OpenAPI spec.

            The OpenAPI spec can be specified as either `spec` given the spec as a JSON object, or as
            `specRaw` (a `string`) which will be parsed as YAML or JSON by the server. Sending neither
            or both is invalid, resulting in a `400` **Bad Request**.

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        EventTypeImportOpenApiOut
    """

    return (
        await request_asyncio_detailed(
            client=client,
            json_body=json_body,
            idempotency_key=idempotency_key,
        )
    ).parsed
