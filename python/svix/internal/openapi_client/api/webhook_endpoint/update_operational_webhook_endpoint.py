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
from ...models.operational_webhook_endpoint_out import OperationalWebhookEndpointOut
from ...models.operational_webhook_endpoint_update import OperationalWebhookEndpointUpdate
from ...types import Response


def _get_kwargs(
    endpoint_id: str,
    *,
    client: AuthenticatedClient,
    json_body: OperationalWebhookEndpointUpdate,
) -> Dict[str, Any]:
    url = "{}/api/v1/operational-webhook/endpoint/{endpoint_id}".format(client.base_url, endpoint_id=endpoint_id)

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


def _parse_response(*, client: Client, response: httpx.Response) -> OperationalWebhookEndpointOut:
    if response.status_code == HTTPStatus.OK:
        response_200 = OperationalWebhookEndpointOut.from_dict(response.json())

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


def _build_response(*, client: Client, response: httpx.Response) -> Response[OperationalWebhookEndpointOut]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def request_sync_detailed(
    endpoint_id: str,
    *,
    client: AuthenticatedClient,
    json_body: OperationalWebhookEndpointUpdate,
) -> Response[OperationalWebhookEndpointOut]:
    """Update Operational Webhook Endpoint

     Update an operational webhook endpoint.

    Args:
        endpoint_id (str): The ep's ID or UID Example: unique-ep-identifier.
        json_body (OperationalWebhookEndpointUpdate):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[OperationalWebhookEndpointOut]
    """

    kwargs = _get_kwargs(
        endpoint_id=endpoint_id,
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
    endpoint_id: str,
    *,
    client: AuthenticatedClient,
    json_body: OperationalWebhookEndpointUpdate,
) -> OperationalWebhookEndpointOut:
    """Update Operational Webhook Endpoint

     Update an operational webhook endpoint.

    Args:
        endpoint_id (str): The ep's ID or UID Example: unique-ep-identifier.
        json_body (OperationalWebhookEndpointUpdate):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        OperationalWebhookEndpointOut
    """

    return request_sync_detailed(
        endpoint_id=endpoint_id,
        client=client,
        json_body=json_body,
    ).parsed


async def request_asyncio_detailed(
    endpoint_id: str,
    *,
    client: AuthenticatedClient,
    json_body: OperationalWebhookEndpointUpdate,
) -> Response[OperationalWebhookEndpointOut]:
    """Update Operational Webhook Endpoint

     Update an operational webhook endpoint.

    Args:
        endpoint_id (str): The ep's ID or UID Example: unique-ep-identifier.
        json_body (OperationalWebhookEndpointUpdate):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[OperationalWebhookEndpointOut]
    """

    kwargs = _get_kwargs(
        endpoint_id=endpoint_id,
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
    endpoint_id: str,
    *,
    client: AuthenticatedClient,
    json_body: OperationalWebhookEndpointUpdate,
) -> OperationalWebhookEndpointOut:
    """Update Operational Webhook Endpoint

     Update an operational webhook endpoint.

    Args:
        endpoint_id (str): The ep's ID or UID Example: unique-ep-identifier.
        json_body (OperationalWebhookEndpointUpdate):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        OperationalWebhookEndpointOut
    """

    return (
        await request_asyncio_detailed(
            endpoint_id=endpoint_id,
            client=client,
            json_body=json_body,
        )
    ).parsed
