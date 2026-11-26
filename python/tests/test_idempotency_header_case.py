import httpx
import pytest

from svix.api import ApplicationCreateOptions, ApplicationIn
from svix.api.application import Application, ApplicationAsync
from svix.api.client import AuthenticatedClient

CASES = [
    ("Idempotency-Key", None, "configured"),
    ("IDEMPOTENCY-KEY", None, "configured"),
    ("idempotency-key", None, "configured"),
    ("Idempotency-Key", "request-key", "request-key"),
    (None, None, None),
]
RESPONSE = {
    "id": "app_test",
    "name": "Test",
    "metadata": {},
    "createdAt": "2026-01-01T00:00:00Z",
    "updatedAt": "2026-01-01T00:00:00Z",
}


def check_key(request: httpx.Request, expected: str | None) -> None:
    keys = request.headers.get_list("idempotency-key")
    assert len(keys) == 1
    if expected is None:
        assert keys[0].startswith("auto_")
    else:
        assert keys == [expected]


def configuration(name):
    return AuthenticatedClient(
        base_url="https://api.svix.test",
        token="test",
        retry_schedule=[],
        headers={name: "configured"} if name else {},
    )


@pytest.mark.parametrize("name,override,expected", CASES)
def test_sync_idempotency_header_case(httpx_mock, name, override, expected):
    httpx_mock.add_response(json=RESPONSE)
    with httpx.Client() as transport:
        Application(configuration(name), transport).create(
            ApplicationIn(name="Test"),
            ApplicationCreateOptions(idempotency_key=override),
        )
    check_key(httpx_mock.get_request(), expected)


@pytest.mark.asyncio
@pytest.mark.parametrize("name,override,expected", CASES)
async def test_async_idempotency_header_case(httpx_mock, name, override, expected):
    httpx_mock.add_response(json=RESPONSE)
    async with httpx.AsyncClient() as transport:
        await ApplicationAsync(configuration(name), transport).create(
            ApplicationIn(name="Test"),
            ApplicationCreateOptions(idempotency_key=override),
        )
    check_key(httpx_mock.get_request(), expected)
