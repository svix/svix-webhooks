import itertools
import uuid
from typing import Any, Dict, List, Optional

import pytest
from pytest_httpserver import HTTPServer
from werkzeug import Request, Response

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
from svix.webhooks import Webhook


def _gen_uuid(name: str) -> str:
    return str(uuid.uuid5(uuid.NAMESPACE_DNS, name))


@pytest.fixture
def svix_app_name() -> str:
    return "svix_python_tests"


@pytest.fixture
def event_type_schema() -> Dict[str, Any]:
    return {
        "type": "object",
        "title": "event.test",
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
            version=1,
            filter_types=[event_type_name],
            channels=[channel] if channel else None,
            metadata=metadata,  # type: ignore[arg-type]
            secret=secret,
        ),
    )


def test_svix_application_create(svix_api: Svix, svix_app_name: str) -> None:
    svix_app_uid = _gen_uuid(svix_app_name)
    app = create_svix_app(svix_api, svix_app_name, svix_app_uid)
    assert app.name == svix_app_name
    assert app.uid == svix_app_uid


def test_svix_event_type_create(
    svix_api: Svix, event_type_schema: Dict[str, Any]
) -> None:
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


@pytest.mark.parametrize(
    "with_channel,with_metadata,with_secret",
    list(itertools.product([False, True], repeat=3)),
    ids=svix_endpoint_create_test_params_ids(),
)
def test_svix_endpoint_create(
    svix_api: Svix,
    svix_app_name: str,
    event_type_schema: Dict[str, Any],
    endpoint_url: str,
    with_channel: bool,
    with_metadata: bool,
    with_secret: bool,
) -> None:
    svix_app_uid = _gen_uuid(svix_app_name)
    app = create_svix_app(svix_api, svix_app_name, svix_app_uid)
    event_type = create_svix_event_type(svix_api, event_type_schema)
    endpoint_uid = _gen_uuid(endpoint_url)
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
    assert endpoint.filter_types == [event_type.name]
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
    event_type_schema: Dict[str, Any],
    httpserver: HTTPServer,
    with_channel: bool,
) -> None:
    svix_app_uid = _gen_uuid(svix_app_name)
    create_svix_app(svix_api, svix_app_name, svix_app_uid)
    event_type = create_svix_event_type(svix_api, event_type_schema)

    channel = "test" if with_channel else None
    endpoint_path = "/webhook/receiver/"
    endpoint_url = httpserver.url_for(endpoint_path)
    endpoint_uid = _gen_uuid(endpoint_url)
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
        received_payload = webhook.verify(request.data, request.headers)
        assert received_payload == payload

        return Response("OK")

    httpserver.expect_oneshot_request(
        endpoint_path,
        method="POST",
        json=payload,
    ).respond_with_handler(webhook_handler)

    # send message and check it is received by local http server
    with httpserver.wait() as waiting:
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
