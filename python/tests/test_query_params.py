import asyncio
from datetime import datetime, timedelta, timezone

import pytest
from pytest_httpx import HTTPXMock

from svix.api import MessageListOptions, Svix, SvixAsync, SvixOptions


@pytest.mark.parametrize("async_client", [False, True])
@pytest.mark.parametrize("field", ["before", "after"])
@pytest.mark.parametrize(
    "value, expected",
    [
        (datetime(2026, 1, 2, 3, 4, 5), "2026-01-02T03:04:05+00:00"),
        (
            datetime(2026, 1, 2, 3, 4, 5, tzinfo=timezone.utc),
            "2026-01-02T03:04:05+00:00",
        ),
        (
            datetime(
                2026, 1, 2, 3, 4, 5, tzinfo=timezone(timedelta(hours=5, minutes=30))
            ),
            "2026-01-02T03:04:05+05:30",
        ),
    ],
)
def test_message_list_datetime_filters(
    httpx_mock: HTTPXMock,
    async_client: bool,
    field: str,
    value: datetime,
    expected: str,
) -> None:
    httpx_mock.add_response(
        json={"data": [], "done": True, "iterator": None, "prevIterator": None}
    )
    options = (
        MessageListOptions(before=value)
        if field == "before"
        else MessageListOptions(after=value)
    )
    original_tz = value.tzinfo
    if async_client:

        async def run() -> None:
            client = SvixAsync(
                "token", SvixOptions(server_url="https://api.example.test")
            )
            try:
                await client.message.list("app_test", options)
            finally:
                await client._httpx_client.aclose()

        asyncio.run(run())
    else:
        client = Svix("token", SvixOptions(server_url="https://api.example.test"))
        try:
            client.message.list("app_test", options)
        finally:
            client._httpx_client.close()
    assert httpx_mock.get_requests()[-1].url.params[field] == expected
    assert value.tzinfo is original_tz
