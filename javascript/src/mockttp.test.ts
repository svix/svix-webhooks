import { BackgroundTaskType, MessageAttemptTriggerType, Svix } from ".";
import { test } from "node:test";
import { strict as assert } from "node:assert/strict";
import * as mockttp from "mockttp";
import { ApiException } from "./util";
import { Ordering } from "./models/ordering";
import type { ValidationError, HttpErrorOut } from "./HttpErrors";
import { LIB_VERSION } from "./request";

const ApplicationOut = `{"uid":"unique-identifier","name":"My first application","rateLimit":0,"id":"app_1srOrx2ZWZBpBUvZwXKQmoEYga2","createdAt":"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z","metadata":{"property1":"string","property2":"string"}}`;
const ListResponseMessageOut = `{"data":[{"eventId":"unique-identifier","eventType":"user.signup","payload":{"email":"test@example.com","type":"user.created","username":"test_user"},"channels":["project_123","group_2"],"id":"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2","timestamp":"2019-08-24T14:15:22Z","tags":["project_1337"]}],"iterator":"iterator","prevIterator":"-iterator","done":true}`;
const ListResponseApplicationOut = `{"data":[{"uid":"unique-identifier","name":"My first application","rateLimit":0,"id":"app_1srOrx2ZWZBpBUvZwXKQmoEYga2","createdAt":"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z","metadata":{"property1":"string","property2":"string"}}],"iterator":"iterator","prevIterator":"-iterator","done":true}`;
const ListResponseMessageAttemptOut = `{"data":[{"url":"https://example.com/webhook/","response":"{}","responseStatusCode":200,"responseDurationMs":0,"status":0,"triggerType":0,"msgId":"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2","endpointId":"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2","id":"atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2","timestamp":"2025-02-16T21:38:21.977Z","msg":{"eventId":"unique-identifier","eventType":"user.signup","payload":{"email":"test@example.com","type":"user.created","username":"test_user"},"channels":["project_123","group_2"],"id":"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2","timestamp":"2025-02-16T21:38:21.977Z","tags":["project_1337"]}}],"iterator":"iterator","prevIterator":"-iterator","done":true}`;
const ListResponseOperationalWebhookEndpointOut = `{"data":[{"id":"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2","description":"string","rateLimit":0,"uid":"unique-identifier","url":"https://example.com/webhook/","disabled":false,"filterTypes":["message.attempt.failing"],"createdAt":"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z","metadata":{"property1":"string","property2":"string"}}],"iterator":"iterator","prevIterator":"-iterator","done":true}`;
const ListResponseBackgroundTaskOut = `{"data":[{"data":{},"id":"qtask_1srOrx2ZWZBpBUvZwXKQmoEYga2","status":"running","task":"endpoint.replay"}],"iterator":"iterator","prevIterator":"-iterator","done":true}`;
const EventTypeImportOpenApiOut = `{"data":{"modified":["user.signup"],"to_modify":[{"name":"user.signup","description":"string","schemas":{},"deprecated":true,"featureFlag":"cool-new-feature","groupName":"user"}]}}`;
const ReplayOut = `{"id":"qtask_1srOrx2ZWZBpBUvZwXKQmoEYga2","status":"running","task":"endpoint.replay"}`;
const EndpointOut = `{"description":"string","rateLimit":0,"uid":"unique-identifier","url":"http://example.com","version":1,"disabled":true,"filterTypes":["user.signup"],"channels":["project_1337"],"secret":"whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD","metadata":{"property1":"string","property2":"string"}}`;
const ValidationErrorOut = `{"detail":[{"loc":["string"],"msg":"string","type":"string"}]}`;
const IngestSourceOutCron = `{"type":"cron","config":{"schedule":"hello","payload":"world"},"id":"src_2yZwUhtgs5Ai8T9yRQJXA","uid":"unique-identifier","name":"string","ingestUrl":"http://example.com","createdAt":"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z"}`;
const IngestSourceOutGeneric = `{"type":"generic-webhook","config":{},"id":"src_2yZwUhtgs5Ai8T9yRQJXA","uid":"unique-identifier","name":"string","ingestUrl":"http://example.com","createdAt":"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z"}`;
const mockServer = mockttp.getLocal();

test("mockttp tests", async (t) => {
  t.beforeEach(async () => await mockServer.start(0));
  t.afterEach(async () => await mockServer.stop());

  await t.test("test Date in query param", async () => {
    const endpointMock = await mockServer
      .forGet(/\/api\/v1\/app\/app1\/attempt\/endpoint\/ep1.*/)
      .thenReply(200, ListResponseMessageAttemptOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.messageAttempt.listByEndpoint("app1", "ep1", {
      before: new Date(1739741901977),
    });
    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert(requests[0].url.endsWith("before=2025-02-16T21%3A38%3A21.977Z"));
  });

  await t.test("test Date request body", async () => {
    const endpointMock = await mockServer
      .forPost("/api/v1/app/app1/endpoint/ep1/replay-missing")
      .thenReply(200, ReplayOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.endpoint.replayMissing("app1", "ep1", { since: new Date(1739741901977) });
    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert.equal(
      await requests[0].body.getText(),
      `{"since":"2025-02-16T21:38:21.977Z"}`
    );
  });

  await t.test("test Date response body", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app/app1/attempt/endpoint/ep1")
      .thenReply(200, ListResponseMessageAttemptOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const res = await svx.messageAttempt.listByEndpoint("app1", "ep1");
    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert.equal(res.data[0].timestamp?.getTime(), 1739741901977);
  });

  await t.test("test listResponseOut deserializes correctly", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app/app1/attempt/endpoint/ep1")
      .thenReply(200, `{"data":[],"iterator":null,"prevIterator":null,"done":true}`);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const res = await svx.messageAttempt.listByEndpoint("app1", "ep1");
    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert.equal(res.iterator, null);
    assert.equal(res.prevIterator, null);
  });

  await t.test("test enum as query param", async () => {
    const endpointMock = await mockServer
      .forGet(/\/api\/v1\/app.*/)
      .thenReply(200, ListResponseApplicationOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.application.list({ order: Ordering.Ascending });
    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert(requests[0].url.endsWith("order=ascending"));
  });

  await t.test("test list as query param", async () => {
    const endpointMock = await mockServer
      .forGet(/\/api\/v1\/app\/app1\/msg.*/)
      .thenReply(200, ListResponseMessageOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.message.list("app1", { eventTypes: ["val8", "val1", "val5"] });
    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert(
      requests[0].url.endsWith("/api/v1/app/app1/msg?event_types=val8%2Cval1%2Cval5")
    );
  });

  await t.test("test header param sent", async () => {
    const endpointMock = await mockServer
      .forPost(/\/api\/v1\/app.*/)
      .thenReply(200, ApplicationOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.application.create({ name: "test" }, { idempotencyKey: "random-key" });
    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert.equal(requests[0].headers["idempotency-key"], "random-key");
  });

  await t.test("test retry for status => 500", async () => {
    const numRetries = 5;
    const endpointMock = await mockServer
      .forGet("/api/v1/app")
      .thenReply(500, `{"code":"500","detail":"asd"}`);
    const svx = new Svix("token", {
      serverUrl: mockServer.url,
      numRetries,
    });

    await assert.rejects(svx.application.list(), ApiException);

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, numRetries + 1);

    // same svix-req-id for each retry
    const req_id = requests[0].headers["svix-req-id"];
    assert(req_id);
    assert.equal(typeof req_id, "string");
    for (let i = 0; i < requests.length; i++) {
      assert.equal(requests[i].headers["svix-req-id"], req_id);
      if (i === 0) {
        // first request does not set svix-retry-count
        assert.equal(requests[i].headers["svix-retry-count"], undefined);
      } else {
        assert.equal(requests[i].headers["svix-retry-count"], i.toString());
      }
    }
  });

  await t.test("test retry schedule for status => 500", async () => {
    const retryScheduleInMs = [60, 120, 240];
    const endpointMock = await mockServer
      .forGet("/api/v1/app")
      .thenReply(500, `{"code":"500","detail":"asd"}`);
    const before = Date.now();
    const svx = new Svix("token", {
      serverUrl: mockServer.url,
      retryScheduleInMs,
    });

    await assert.rejects(svx.application.list(), ApiException);

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, retryScheduleInMs.length + 1);

    const after = Date.now();

    assert(after - before >= retryScheduleInMs.reduce((prev, curr) => prev + curr, 0));
  });

  await t.test("no body in response does not return anything", async () => {
    const endpointMock = await mockServer
      .forDelete("/api/v1/app/app1")
      .thenReply(204, "");
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.application.delete("app1");

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
  });

  await t.test("422 returns validation error", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app")
      .thenReply(422, ValidationErrorOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await assert.rejects(svx.application.list(), ApiException<ValidationError>);
    try {
      await svx.application.list();
    } catch (e) {
      const error = e as ApiException<ValidationError>;
      assert.equal(error.code, 422);
      assert.deepEqual(error.body, {
        detail: [{ loc: ["string"], msg: "string", type: "string" }],
      });
    }

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 2);
  });

  await t.test("400 returns ApiException", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app")
      .thenReply(400, `{"code":"400","detail":"text"}`);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await assert.rejects(svx.application.list(), ApiException<HttpErrorOut>);
    try {
      await svx.application.list();
    } catch (e) {
      const error = e as ApiException<HttpErrorOut>;
      assert.equal(error.code, 400);
      assert.deepEqual(error.body, { code: "400", detail: "text" });
    }

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 2);
  });

  await t.test("sub-resource works", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/operational-webhook/endpoint")
      .thenReply(200, ListResponseOperationalWebhookEndpointOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.operationalWebhookEndpoint.list();

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
  });

  await t.test("integer enum serialization", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app/app1/attempt/endpoint/endp1")
      .thenReply(200, ListResponseMessageAttemptOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const res = await svx.messageAttempt.listByEndpoint("app1", "endp1");
    assert.equal(res.data[0].triggerType, MessageAttemptTriggerType.Scheduled);

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
  });

  await t.test("string enum de/serialization", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/background-task")
      .thenReply(200, ListResponseBackgroundTaskOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const res = await svx.backgroundTask.list({
      task: BackgroundTaskType.EndpointReplay,
    });
    assert.equal(res.data[0].task, BackgroundTaskType.EndpointReplay);

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert(requests[0].url.endsWith("api/v1/background-task?task=endpoint.replay"));
  });

  await t.test("non-camelCase field name", async () => {
    const endpointMock = await mockServer
      .forPost("/api/v1/event-type/import/openapi")
      .thenReply(200, EventTypeImportOpenApiOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const res = await svx.eventType.importOpenapi({ dryRun: true });
    assert.deepEqual(res.data.toModify, [
      {
        name: "user.signup",
        description: "string",
        schemas: {},
        deprecated: true,
        featureFlag: "cool-new-feature",
        featureFlags: undefined,
        groupName: "user",
      },
    ]);

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
  });

  await t.test("patch request body", async () => {
    const endpointMock = await mockServer
      .forPatch("/api/v1/app/app1/endpoint/endp1")
      .thenReply(200, EndpointOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.endpoint.patch("app1", "endp1", { filterTypes: ["ty1"] });
    await svx.endpoint.patch("app1", "endp1", { filterTypes: null });
    await svx.endpoint.patch("app1", "endp1", { description: "text" });

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 3);

    // nullable field is sent
    assert.equal(await requests[0].body.getText(), `{"filterTypes":["ty1"]}`);
    // nullable field is null
    assert.equal(await requests[1].body.getText(), `{"filterTypes":null}`);
    // undefined field is omitted
    assert.equal(await requests[2].body.getText(), `{"description":"text"}`);
  });

  await t.test("arbitrary json object body", async () => {
    const endpointMock = await mockServer
      .forPost("/api/v1/app/app1/msg")
      .thenReply(200, EndpointOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.message.create("app1", {
      eventType: "asd",
      payload: { key1: "val", list: ["val1"], obj: { key: "val2" } },
    });

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert.equal(
      await requests[0].body.getText(),
      `{"eventType":"asd","payload":{"key1":"val","list":["val1"],"obj":{"key":"val2"}}}`
    );
  });

  await t.test("token/user-agent is sent", async () => {
    const endpointMock = await mockServer
      .forDelete("/api/v1/app/app1")
      .thenReply(200, EndpointOut);
    const svx = new Svix("token.eu", { serverUrl: mockServer.url });

    await svx.application.delete("app1");

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert.equal(requests[0].headers["authorization"], "Bearer token.eu");
    assert.equal(
      requests[0].headers["user-agent"],
      `svix-libs/${LIB_VERSION}/javascript`
    );
  });

  await t.test("MessageAttemptOut without msg", async () => {
    const RES = `{
      "data": [
        {
          "url": "https://example.com/webhook/",
          "response": "{}",
          "responseStatusCode": 200,
          "responseDurationMs": 0,
          "status": 0,
          "triggerType": 0,
          "msgId": "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2",
          "endpointId": "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2",
          "id": "atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2",
          "timestamp": "2019-08-24T14:15:22Z"
        }
      ],
      "iterator": "iterator",
      "prevIterator": "-iterator",
      "done": true
    }`;
    const endpointMock = await mockServer
      .forGet("/api/v1/app/app/attempt/endpoint/edp")
      .thenReply(200, RES);

    const svx = new Svix("token.eu", { serverUrl: mockServer.url });

    await svx.messageAttempt.listByEndpoint("app", "edp");

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
  });

  await t.test("octothorpe in url query", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app/app1/msg")
      .thenReply(200, ListResponseMessageOut);
    const svx = new Svix("token.eu", { serverUrl: mockServer.url });

    await svx.message.list("app1", { tag: "test#test" });

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert(requests[0].url.endsWith("api/v1/app/app1/msg?tag=test%23test"));
  });

  await t.test("content-type application/json is sent on request with body", async () => {
    const endpointMock = await mockServer
      .forPost(/\/api\/v1\/app.*/)
      .thenReply(200, ApplicationOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.application.create({ name: "test" });
    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert.equal(requests[0].headers["content-type"], "application/json");
  });

  await t.test("content type not sent on request without body", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app")
      .thenReply(200, ListResponseApplicationOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.application.list();
    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert.equal(requests[0].headers["content-type"], undefined);
  });

  await t.test("struct enum with extra fields", async () => {
    const endpointMock = await mockServer
      .forPost("/ingest/api/v1/source")
      .thenReply(200, IngestSourceOutCron);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const res = await svx.ingest.source.create({
      name: "crontab -r",
      type: "cron",
      config: { schedule: "hello", payload: "world" },
    });

    assert.equal(res.type, "cron");
    // this will smart cast res.config
    if (res.type === "cron") {
      assert.equal(res.config.schedule, "hello");
      assert.equal(res.config.payload, "world");
    }
    const requests = await endpointMock.getSeenRequests();
    assert.equal(
      await requests[0].body.getText(),
      '{"type":"cron","config":{"payload":"world","schedule":"hello"},"name":"crontab -r"}'
    );
  });

  await t.test("struct enum with no extra fields", async () => {
    const endpointMock = await mockServer
      .forPost("/ingest/api/v1/source")
      .thenReply(200, IngestSourceOutGeneric);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const res = await svx.ingest.source.create({
      name: "generic over <T>",
      type: "generic-webhook",
    });

    assert.equal(res.type, "generic-webhook");
    const requests = await endpointMock.getSeenRequests();
    // empty config object should be sent
    assert.equal(
      await requests[0].body.getText(),
      '{"type":"generic-webhook","config":{},"name":"generic over <T>"}'
    );
  });

  await t.test("test idempotency key is sent for create request", async () => {
    const endpointMock = await mockServer
      .forPost("/api/v1/app")
      .thenReply(200, ApplicationOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.application.create({ name: "test app" });

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
    const idempotencyKey = requests[0].headers["idempotency-key"] as string;
    assert(idempotencyKey.startsWith("auto_"));
  });

  await t.test("test client provided idempotency key is not overridden", async () => {
    const endpointMock = await mockServer
      .forPost("/api/v1/app")
      .thenReply(200, ApplicationOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const clientProvidedKey = "test-key-123";
    await svx.application.create(
      { name: "test app" },
      { idempotencyKey: clientProvidedKey }
    );

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert.equal(requests[0].headers["idempotency-key"], clientProvidedKey);
  });

  await t.test("test unknown keys are ignored", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app")
      .thenReply(
        200,
        `{"data":[],"done":true,"iterator":null,"prevIterator":null,"extra-key":"ignored"}`
      );
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.application.list();

    const requests = await endpointMock.getSeenRequests();
    assert.equal(requests.length, 1);
  });

  await t.test("should use custom fetch implementation", async () => {
    let customFetchCalled = false;
    const mockFetch: typeof fetch = async (_input, _init) => {
      customFetchCalled = true;
      return new Response(ListResponseApplicationOut, {
        status: 200,
        headers: { "Content-Type": "application/json" },
      });
    };

    await mockServer
      .forGet(/\/api\/v1\/app.*/)
      .thenReply(200, ListResponseApplicationOut);
    const svx = new Svix("token", { serverUrl: mockServer.url, fetch: mockFetch });
    await svx.application.list({ order: Ordering.Ascending });
    assert(customFetchCalled);
  });
});
