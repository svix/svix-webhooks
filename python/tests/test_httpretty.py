import httpretty

import svix
from svix.api import (
    ApplicationCreateOptions,
    ApplicationIn,
    CronConfig,
    IngestSourceIn,
    MessageIn,
    MessageListOptions,
)


@httpretty.activate(verbose=True, allow_net_connect=False)
def test_octothorpe_in_query_param():
    svx = svix.Svix("token", svix.SvixOptions(server_url="http://test.example"))
    httpretty.register_uri(
        httpretty.GET,
        "http://test.example/api/v1/app/app_id/msg?tag=test%23test",
        body='{"data":[],"iterator":null,"prevIterator":null,"done":true}',
    )
    svx.message.list("app_id", MessageListOptions(tag="test#test"))


@httpretty.activate(verbose=True, allow_net_connect=False)
def test_struct_enum_with_fields():
    svx = svix.Svix("token", svix.SvixOptions(server_url="http://test.example"))
    httpretty.register_uri(
        httpretty.POST,
        "http://test.example/ingest/api/v1/source",
        body=(
            '{"type":"cron","config":{"content_type":"mendy/tired","payload":"@hello '
            'there","schedule":"* * * * *"},"id":"src_2yZwUhtgs5Ai8T9yRQJXA","uid":"'
            'unique-identifier","name":"string","ingestUrl":'
            '"http://example.com","createdAt":"2019-08-24T14:15:22Z",'
            '"updatedAt":"2019-08-24T14:15:22Z",'
            '"metadata":{ }}'
        ),
    )
    source_in = IngestSourceIn(
        name="name",
        type="cron",
        config=CronConfig(content_type="asd", payload="asd", schedule="asd"),
    )
    res = svx.ingest.source.create(source_in)
    assert isinstance(res.config, CronConfig)

    assert res.config.content_type == "mendy/tired"
    assert res.config.payload == "@hello there"
    assert res.config.schedule == "* * * * *"


@httpretty.activate(verbose=True, allow_net_connect=False)
def test_struct_enum_without_fields():
    svx = svix.Svix("token", svix.SvixOptions(server_url="http://test.example"))
    httpretty.register_uri(
        httpretty.POST,
        "http://test.example/ingest/api/v1/source",
        body=(
            '{"type":"generic-webhook","id":"src_2yZwUhtgs5Ai8T9yRQJXA","uid":"unique-'
            'identifier","name":"string","ingestUrl":"http://example.com","createdAt":'
            '"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z","metadata":{ }}'
        ),
    )
    source_in = IngestSourceIn(name="name", type="generic-webhook", config={})

    res = svx.ingest.source.create(source_in)

    assert isinstance(res.config, dict)
    assert res.config == {}


@httpretty.activate(verbose=True, allow_net_connect=False)
def test_idempotency_key_is_sent_for_create_request():
    svx = svix.Svix("token", svix.SvixOptions(server_url="http://test.example"))
    httpretty.register_uri(
        httpretty.POST,
        "http://test.example/api/v1/app",
        body=(
            '{"uid":"unique-identifier","name":"My first application","throttleRate":0,'
            '"id":"app_1srOrx2ZWZBpBUvZwXKQmoEYga2","createdAt":"2019-08-24T14:15:22Z"'
            ',"updatedAt":"2019-08-24T14:15:22Z","metadata":{"property1":"string",'
            '"property2":"string"}}'
        ),
    )
    svx.application.create(ApplicationIn(name="test app"))

    request = httpretty.latest_requests()[0]
    assert "idempotency-key" in request.headers
    assert request.headers["idempotency-key"].startswith("auto_")


@httpretty.activate(verbose=True, allow_net_connect=False)
def test_client_provided_idempotency_key_is_not_overridden():
    svx = svix.Svix("token", svix.SvixOptions(server_url="http://test.example"))
    httpretty.register_uri(
        httpretty.POST,
        "http://test.example/api/v1/app",
        body=(
            '{"uid":"unique-identifier","name":"My first application","throttleRate":0'
            ',"id":"app_1srOrx2ZWZBpBUvZwXKQmoEYga2","createdAt":"2019-08-24T14:15:'
            '22Z","updatedAt":"2019-08-24T14:15:22Z","metadata":{"property1":"string"'
            ',"property2":"string"}}'
        ),
    )

    client_provided_key = "test-key-123"
    svx.application.create(
        ApplicationIn(name="test app"),
        ApplicationCreateOptions(idempotency_key=client_provided_key),
    )

    request = httpretty.latest_requests()[0]
    assert "idempotency-key" in request.headers
    assert request.headers["idempotency-key"] == client_provided_key


@httpretty.activate(verbose=True, allow_net_connect=False)
def test_unknown_keys_are_ignored():
    svx = svix.Svix("token", svix.SvixOptions(server_url="http://test.example"))
    httpretty.register_uri(
        httpretty.GET,
        "http://test.example/api/v1/app",
        body='{"data":[],"done":true,"iterator":null,"prevIterator":null,"extra-key":"ignored"}',
    )

    svx.application.list()


@httpretty.activate(verbose=True, allow_net_connect=False)
def test_cmg_with_content_default():
    svx = svix.Svix("token", svix.SvixOptions(server_url="http://test.example"))

    app_id = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2"
    httpretty.register_uri(
        httpretty.POST,
        f"http://test.example/api/v1/app/{app_id}/msg",
        status=202,
        body=(
            """{
                "channels": null,
                "deliverAt": null,
                "eventId": null,
                "eventType": "user.signup",
                "id": "msg_2srOrx2ZWZBpBUvZwXKQmoEYga2",
                "payload": { "m": "FILTERED" },
                "tags": null,
                "timestamp": "2026-06-08T09:25:17.864Z"
            }"""
        ),
    )

    payload = {
        "type": "user.signup",
        "email": "test@example.com",
        "username": "test_user",
    }
    response = svx.message.create(
        app_id, MessageIn(event_type="user.signup", payload=payload)
    )

    assert response.payload == {"m": "FILTERED"}
    reqs = httpretty.latest_requests()
    assert reqs[0].url.endswith("/msg?with_content=false")
