import hashlib
import itertools
import platform
import uuid
from typing import Any, Dict, List, Optional

import httpx
import pytest
from pytest_httpserver import HTTPServer
from werkzeug import Request, Response

from svix import __version__
from svix.api import (
    ApplicationIn,
    ApplicationOut,
    EndpointIn,
    EndpointOut,
    EventTypeIn,
    EventTypeOut,
    MessageIn,
    Svix,
)
from svix.api.errors.http_error import HttpError
from svix.api.errors.http_validation_error import HTTPValidationError
from svix.webhooks import Webhook


def _gen_uid(name: str) -> str:
    hasher = hashlib.sha256()
    hasher.update(name.encode("utf-8"))
    hasher.update(str(uuid.uuid4()).encode("ascii"))
    return hasher.hexdigest()


@pytest.fixture
def svix_app_name() -> str:
    return "svix_python_tests"


def event_type_name() -> str:
    return f"event.{str(uuid.uuid4()).replace('-', '')}"


def make_event_type_schema(event_type_name) -> Dict[str, Any]:
    return {
        "type": "object",
        "title": event_type_name,
        "description": "A dummy event type",
        "properties": {
            "value": {
                "type": "string",
                "description": "A simple string value",
            }
        },
        "required": ["value"],
    }


@pytest.fixture
def endpoint_url() -> str:
    return "http://localhost/webhook/receiver"


def create_svix_app(
    svix_api: Svix, svix_app_name: str, svix_app_uid: str
) -> ApplicationOut:
    return svix_api.application.get_or_create(
        ApplicationIn(name=svix_app_name, uid=svix_app_uid)
    )


def create_svix_event_type(
    svix_api: Svix, event_type_schema: Dict[str, Any]
) -> EventTypeOut:
    return svix_api.event_type.create(
        EventTypeIn(
            name=event_type_schema["title"],
            description=event_type_schema["description"],
            schemas={"1": event_type_schema},
        )
    )


def create_svix_endpoint(
    svix_api: Svix,
    app_uid: str,
    event_type_name: str,
    endpoint_url: str,
    endpoint_uid: str,
    channel: Optional[str] = None,
    metadata: Optional[Dict[str, Any]] = None,
    secret: Optional[str] = None,
) -> EndpointOut:
    return svix_api.endpoint.create(
        app_uid,
        EndpointIn(
            url=endpoint_url,
            uid=endpoint_uid,
            event_types=[event_type_name],
            channels=[channel] if channel else None,
            metadata=metadata,
            secret=secret,
        ),
    )


def test_svix_application_create(svix_api: Svix, svix_app_name: str) -> None:
    svix_app_uid = _gen_uid(svix_app_name)
    app = create_svix_app(svix_api, svix_app_name, svix_app_uid)
    assert app.name == svix_app_name
    assert app.uid == svix_app_uid


def test_svix_event_type_create(svix_api: Svix) -> None:
    event_type_schema = make_event_type_schema(event_type_name())
    event_type = create_svix_event_type(svix_api, event_type_schema)
    assert event_type.name == event_type_schema["title"]
    assert event_type.description == event_type_schema["description"]
    assert event_type.schemas == {"1": event_type_schema}


def svix_endpoint_create_test_params_ids() -> List[str]:
    ids = []
    for params in itertools.product([False, True], repeat=3):
        with_channel, with_metadata, with_secret = params
        ids.append(
            "/".join(
                [
                    ("with" if with_channel else "without") + " channel",
                    ("with" if with_metadata else "without") + " metadata",
                    ("with" if with_secret else "without") + " secret",
                ]
            )
        )
    return ids


def test_svix_basic_headers(httpx_mock):
    svix_api = Svix(auth_token="not-a-real-auth-token")
    httpx_mock.add_response(
        url="https://api.svix.com/api/v1/app",
        json={
            "data": [],
            "done": True,
            "iterator": "app_3EXsi4X5ClkrXn5GqkzvpnNvwqj",
            "prevIterator": "-app_3EXsi4X5ClkrXn5GqkzvpnNvwqj",
        },
    )
    svix_api.application.list()
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


def test_retries_on_timeout(httpx_mock):
    svix_api = Svix(auth_token="not-a-real-auth-token")
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
    res = svix_api.application.list()
    assert res.iterator == "app_3EXsi4X5ClkrXn5GqkzvpnNvwqj"
    assert len(calls) == 2


def test_retries_on_dns_error(httpx_mock):
    svix_api = Svix(auth_token="not-a-real-auth-token")
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
    res = svix_api.application.list()
    assert res.iterator == "app_3EXsi4X5ClkrXn5GqkzvpnNvwqj"
    assert len(calls) == 2


def test_retries_on_http_500(httpx_mock):
    svix_api = Svix(auth_token="not-a-real-auth-token")
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
    res = svix_api.application.list()
    assert res.iterator == "app_3EXsi4X5ClkrXn5GqkzvpnNvwqj"
    assert len(calls) == 2


def test_retries_on_http_423(httpx_mock):
    svix_api = Svix(auth_token="not-a-real-auth-token")
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
    res = svix_api.application.list()
    assert res.iterator == "app_3EXsi4X5ClkrXn5GqkzvpnNvwqj"
    assert len(calls) == 2


def test_does_not_retry_on_http_409(httpx_mock):
    svix_api = Svix(auth_token="not-a-real-auth-token")

    httpx_mock.add_response(
        url="https://api.svix.com/api/v1/app",
        status_code=409,
        json={
            "code": "CONFLICT",
            "detail": "Sorry, someone else already ate this cake",
        },
    )
    with pytest.raises(HttpError):
        svix_api.application.list()


def test_does_not_retry_on_http_422(httpx_mock):
    svix_api = Svix(auth_token="not-a-real-auth-token")

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
        svix_api.application.list()


@pytest.mark.parametrize(
    "with_channel,with_metadata,with_secret",
    list(itertools.product([False, True], repeat=3)),
    ids=svix_endpoint_create_test_params_ids(),
)
def test_svix_endpoint_create(
    svix_api: Svix,
    svix_app_name: str,
    endpoint_url: str,
    with_channel: bool,
    with_metadata: bool,
    with_secret: bool,
) -> None:
    svix_app_uid = _gen_uid(svix_app_name)
    app = create_svix_app(svix_api, svix_app_name, svix_app_uid)
    event_type_schema = make_event_type_schema(event_type_name())
    event_type = create_svix_event_type(svix_api, event_type_schema)
    endpoint_uid = _gen_uid(endpoint_url)
    channel = "test" if with_channel else None
    metadata = {"test": "test"} if with_metadata else None
    secret = "whsec_" + "e" * 32 if with_secret else None
    assert app.uid
    endpoint = create_svix_endpoint(
        svix_api,
        app.uid,
        event_type.name,
        endpoint_url,
        endpoint_uid,
        channel,
        metadata,
        secret,
    )
    assert endpoint.url == endpoint_url
    assert endpoint.uid == endpoint_uid
    assert endpoint.event_types == [event_type.name]
    if with_channel:
        assert endpoint.channels == [channel]
    if with_metadata:
        assert endpoint.metadata == metadata
    if with_secret:
        assert svix_api.endpoint.get_secret(app.uid, endpoint_uid).key == secret


@pytest.mark.parametrize(
    "with_channel", [False, True], ids=["without channel", "with channel"]
)
def test_svix_message_create(
    svix_api: Svix,
    svix_app_name: str,
    httpserver: HTTPServer,
    with_channel: bool,
    is_ci: bool,
) -> None:
    svix_app_uid = _gen_uid(svix_app_name)
    create_svix_app(svix_api, svix_app_name, svix_app_uid)
    event_type_schema = make_event_type_schema(event_type_name())
    event_type = create_svix_event_type(svix_api, event_type_schema)

    channel = "test" if with_channel else None
    endpoint_path = "/webhook/receiver/"
    if is_ci:
        endpoint_url = httpserver.url_for(endpoint_path)
    else:
        endpoint_url = f"http://host.docker.internal:{httpserver.port}{endpoint_path}"
    endpoint_uid = _gen_uid(endpoint_url)
    create_svix_endpoint(
        svix_api,
        svix_app_uid,
        event_type.name,
        endpoint_url,
        endpoint_uid,
        channel=channel,
    )
    secret = svix_api.endpoint.get_secret(svix_app_uid, endpoint_uid).key

    payload = {"value": "test"}

    def webhook_handler(request: Request) -> Response:
        assert "Svix-Id" in request.headers
        assert "Svix-Timestamp" in request.headers
        assert "Svix-Signature" in request.headers

        webhook = Webhook(secret)
        headers: Dict[str, str] = dict(request.headers.items())
        webhook.verify(request.data, headers)

        return Response("OK")

    httpserver.expect_oneshot_request(
        endpoint_path,
        method="POST",
        json=payload,
    ).respond_with_handler(webhook_handler)

    # send message and check it is received by local http server
    with httpserver.wait(stop_on_nohandler=True, timeout=10) as waiting:
        message_out = svix_api.message.create(
            svix_app_uid,
            MessageIn(
                event_type=event_type.name,
                payload=payload,
                channels=[channel] if channel else None,
            ),
        )

    assert waiting.result

    httpserver.check()  # type: ignore[no-untyped-call]

    assert message_out.event_type == event_type.name
    assert message_out.event_type == event_type.name
    if with_channel:
        assert message_out.channels == [channel]
