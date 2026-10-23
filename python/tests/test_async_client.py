import platform

import httpx
import pytest

from svix import __version__
from svix.api import (
    SvixAsync,
)
from svix.api.errors.http_error import HttpError
from svix.api.errors.http_validation_error import HTTPValidationError


@pytest.mark.asyncio
async def test_svix_basic_headers(httpx_mock):
    svix_api = SvixAsync(auth_token="not-a-real-auth-token")
    httpx_mock.add_response(
        url="https://api.svix.com/api/v1/app",
        json={
            "data": [],
            "done": True,
            "iterator": "app_3EXsi4X5ClkrXn5GqkzvpnNvwqj",
            "prevIterator": "-app_3EXsi4X5ClkrXn5GqkzvpnNvwqj",
        },
    )
    await svix_api.application.list()
    req = httpx_mock.get_request()
    assert req.headers["authorization"] is not None
    assert req.headers["svix-req-id"] is not None
    assert req.headers["user-agent"] == " ".join(
        [
            f"svix-libs/{__version__}/python",
            f"python/{platform.python_version()}",
            f"{platform.system()}/{platform.machine()}",
        ]
    )


@pytest.mark.asyncio
async def test_retries_on_timeout(httpx_mock):
    svix_api = SvixAsync(auth_token="not-a-real-auth-token")
    calls = []

    def timeout_but_only_once(request: httpx.Request):
        nonlocal calls
        calls.append(request)
        if len(calls) < 2:
            raise httpx.ReadTimeout("mock error")
        return httpx.Response(
            status_code=200,
            json={
                "data": [],
                "done": True,
                "iterator": "app_3EXsi4X5ClkrXn5GqkzvpnNvwqj",
                "prevIterator": "-app_3EXsi4X5ClkrXn5GqkzvpnNvwqj",
            },
        )

    httpx_mock.add_callback(
        timeout_but_only_once,
        url="https://api.svix.com/api/v1/app",
        is_reusable=True,
    )
    res = await svix_api.application.list()
    assert res.iterator == "app_3EXsi4X5ClkrXn5GqkzvpnNvwqj"
    assert len(calls) == 2


@pytest.mark.asyncio
async def test_retries_on_dns_error(httpx_mock):
    svix_api = SvixAsync(auth_token="not-a-real-auth-token")
    calls = []

    def timeout_but_only_once(request: httpx.Request):
        nonlocal calls
        calls.append(request)
        if len(calls) < 2:
            raise httpx.ConnectError(
                "[Errno 8] nodename nor servname provided, or not known"
            )
        return httpx.Response(
            status_code=200,
            json={
                "data": [],
                "done": True,
                "iterator": "app_3EXsi4X5ClkrXn5GqkzvpnNvwqj",
                "prevIterator": "-app_3EXsi4X5ClkrXn5GqkzvpnNvwqj",
            },
        )

    httpx_mock.add_callback(
        timeout_but_only_once,
        url="https://api.svix.com/api/v1/app",
        is_reusable=True,
    )
    res = await svix_api.application.list()
    assert res.iterator == "app_3EXsi4X5ClkrXn5GqkzvpnNvwqj"
    assert len(calls) == 2


@pytest.mark.asyncio
async def test_retries_on_http_500(httpx_mock):
    svix_api = SvixAsync(auth_token="not-a-real-auth-token")
    calls = []

    def timeout_but_only_once(request: httpx.Request):
        nonlocal calls
        calls.append(request)
        if len(calls) < 2:
            return httpx.Response(
                status_code=500,
                json={
                    "code": "INTERNAL_SERVER_ERROR",
                    "detail": "oopsie we did a moopsie",
                },
            )
        return httpx.Response(
            status_code=200,
            json={
                "data": [],
                "done": True,
                "iterator": "app_3EXsi4X5ClkrXn5GqkzvpnNvwqj",
                "prevIterator": "-app_3EXsi4X5ClkrXn5GqkzvpnNvwqj",
            },
        )

    httpx_mock.add_callback(
        timeout_but_only_once,
        url="https://api.svix.com/api/v1/app",
        is_reusable=True,
    )
    res = await svix_api.application.list()
    assert res.iterator == "app_3EXsi4X5ClkrXn5GqkzvpnNvwqj"
    assert len(calls) == 2


@pytest.mark.asyncio
async def test_retries_on_http_423(httpx_mock):
    svix_api = SvixAsync(auth_token="not-a-real-auth-token")
    calls = []

    def timeout_but_only_once(request: httpx.Request):
        nonlocal calls
        calls.append(request)
        if len(calls) < 2:
            return httpx.Response(
                status_code=423,
                json={
                    "code": "LOCKED",
                    "detail": "Another request is currently in your castle",
                },
            )
        return httpx.Response(
            status_code=200,
            json={
                "data": [],
                "done": True,
                "iterator": "app_3EXsi4X5ClkrXn5GqkzvpnNvwqj",
                "prevIterator": "-app_3EXsi4X5ClkrXn5GqkzvpnNvwqj",
            },
        )

    httpx_mock.add_callback(
        timeout_but_only_once,
        url="https://api.svix.com/api/v1/app",
        is_reusable=True,
    )
    res = await svix_api.application.list()
    assert res.iterator == "app_3EXsi4X5ClkrXn5GqkzvpnNvwqj"
    assert len(calls) == 2


@pytest.mark.asyncio
async def test_does_not_retry_on_http_409(httpx_mock):
    svix_api = SvixAsync(auth_token="not-a-real-auth-token")

    httpx_mock.add_response(
        url="https://api.svix.com/api/v1/app",
        status_code=409,
        json={
            "code": "CONFLICT",
            "detail": "Sorry, someone else already ate this cake",
        },
    )
    with pytest.raises(HttpError):
        await svix_api.application.list()


@pytest.mark.asyncio
async def test_does_not_retry_on_http_422(httpx_mock):
    svix_api = SvixAsync(auth_token="not-a-real-auth-token")

    httpx_mock.add_response(
        url="https://api.svix.com/api/v1/app",
        status_code=422,
        json={
            "code": "UNPROCESSIBLE_ENTITY",
            "detail": [
                {
                    "loc": ["__all__", "x"],
                    "msg": "You forgot the x field",
                    "type": "MISSING",
                }
            ],
        },
    )
    with pytest.raises(HTTPValidationError):
        await svix_api.application.list()
