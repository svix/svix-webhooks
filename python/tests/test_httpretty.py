import svix
from svix.api import MessageListOptions,IngestSourceIn, CronConfig, ApplicationIn, ApplicationCreateOptions
from svix.api.ingest_source import IngestSource

import httpretty

@httpretty.activate(verbose=True, allow_net_connect=False)
def test_octothorpe_in_query_param():
    svx = svix.Svix("token",svix.SvixOptions(server_url="http://test.example"))
    httpretty.register_uri(
        httpretty.GET,
        "http://test.example/api/v1/app/app_id/msg?tag=test%23test",
        body='{"data":[],"iterator":null,"prevIterator":null,"done":true}'
    )
    svx.message.list("app_id",MessageListOptions(tag="test#test"))

@httpretty.activate(verbose=True, allow_net_connect=False)
def test_struct_enum_with_fields():
    svx = svix.Svix("token", svix.SvixOptions(server_url="http://test.example"))
    ingest_source = IngestSource(svx._client)
    httpretty.register_uri(
        httpretty.POST,
        "http://test.example/ingest/api/v1/source",
        body='{"type":"cron","config":{"content_type":"mendy/tired","payload":"@hello there","schedule":"* * * * *"},"id":"src_2yZwUhtgs5Ai8T9yRQJXA","uid":"unique-identifier","name":"string","ingestUrl":"http://example.com","createdAt":"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z","metadata":{ }}',
    )
    source_in = IngestSourceIn(
        name="name",
        type="cron",
        config=CronConfig(content_type="asd", payload="asd", schedule="asd"),
    )
    res = ingest_source.create(source_in)
    assert isinstance(res.config, CronConfig)

    assert res.config.content_type == "mendy/tired"
    assert res.config.payload == "@hello there"
    assert res.config.schedule == "* * * * *"


@httpretty.activate(verbose=True, allow_net_connect=False)
def test_struct_enum_without_fields():
    svx = svix.Svix("token", svix.SvixOptions(server_url="http://test.example"))
    ingest_source = IngestSource(svx._client)
    httpretty.register_uri(
        httpretty.POST,
        "http://test.example/ingest/api/v1/source",
        body='{"type":"generic-webhook","config":{},"id":"src_2yZwUhtgs5Ai8T9yRQJXA","uid":"unique-identifier","name":"string","ingestUrl":"http://example.com","createdAt":"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z","metadata":{ }}',
    )
    source_in = IngestSourceIn(name="name", type="generic-webhook", config={})

    res = ingest_source.create(source_in)

    assert isinstance(res.config, dict)
    assert res.config == {}

@httpretty.activate(verbose=True, allow_net_connect=False)
def test_idempotency_key_is_sent_for_create_request():
    svx = svix.Svix("token", svix.SvixOptions(server_url="http://test.example"))
    httpretty.register_uri(
        httpretty.POST,
        "http://test.example/api/v1/app",
        body='{"uid":"unique-identifier","name":"My first application","rateLimit":0,"id":"app_1srOrx2ZWZBpBUvZwXKQmoEYga2","createdAt":"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z","metadata":{"property1":"string","property2":"string"}}'
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
        body='{"uid":"unique-identifier","name":"My first application","rateLimit":0,"id":"app_1srOrx2ZWZBpBUvZwXKQmoEYga2","createdAt":"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z","metadata":{"property1":"string","property2":"string"}}'
    )

    client_provided_key = "test-key-123"
    svx.application.create(
        ApplicationIn(name="test app"),
        ApplicationCreateOptions(idempotency_key=client_provided_key)
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
        body='{"data":[],"done":true,"iterator":null,"prevIterator":null,"extra-key":"ignored"}'
    )


    svx.application.list()
