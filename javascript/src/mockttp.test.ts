import { BackgroundTaskType, MessageAttemptTriggerType, Svix } from ".";
import { expect, test } from "@jest/globals";
import * as mockttp from "mockttp";
import { ApiException } from "./util";
import { Ordering } from "./models/ordering";
import { ValidationError, HttpErrorOut } from "./HttpErrors";
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

describe("mockttp tests", () => {
  beforeEach(async () => await mockServer.start(0));
  afterEach(async () => await mockServer.stop());

  test("test Date in query param", async () => {
    const endpointMock = await mockServer
      .forGet(/\/api\/v1\/app\/app1\/attempt\/endpoint\/ep1.*/)
      .thenReply(200, ListResponseMessageAttemptOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.messageAttempt.listByEndpoint("app1", "ep1", {
      before: new Date(1739741901977),
    });
    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(requests[0].url.endsWith("before=2025-02-16T21%3A38%3A21.977Z")).toBe(true);
  });

  test("test Date request body", async () => {
    const endpointMock = await mockServer
      .forPost("/api/v1/app/app1/endpoint/ep1/replay-missing")
      .thenReply(200, ReplayOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.endpoint.replayMissing("app1", "ep1", { since: new Date(1739741901977) });
    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(await requests[0].body.getText()).toBe(`{"since":"2025-02-16T21:38:21.977Z"}`);
  });

  test("test Date response body", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app/app1/attempt/endpoint/ep1")
      .thenReply(200, ListResponseMessageAttemptOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const res = await svx.messageAttempt.listByEndpoint("app1", "ep1");
    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(res.data[0].timestamp?.getTime()).toBe(1739741901977);
  });

  test("test listResponseOut deserializes correctly", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app/app1/attempt/endpoint/ep1")
      .thenReply(200, `{"data":[],"iterator":null,"prevIterator":null,"done":true}`);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const res = await svx.messageAttempt.listByEndpoint("app1", "ep1");
    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(res.iterator).toBeNull();
    expect(res.prevIterator).toBeNull();
  });

  test("test enum as query param", async () => {
    const endpointMock = await mockServer
      .forGet(/\/api\/v1\/app.*/)
      .thenReply(200, ListResponseApplicationOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.application.list({ order: Ordering.Ascending });
    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(requests[0].url.endsWith("order=ascending")).toBe(true);
  });

  test("test list as query param", async () => {
    const endpointMock = await mockServer
      .forGet(/\/api\/v1\/app\/app1\/msg.*/)
      .thenReply(200, ListResponseMessageOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.message.list("app1", { eventTypes: ["val8", "val1", "val5"] });
    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(
      requests[0].url.endsWith("/api/v1/app/app1/msg?event_types=val8%2Cval1%2Cval5")
    ).toBe(true);
  });

  test("test header param sent", async () => {
    const endpointMock = await mockServer
      .forPost(/\/api\/v1\/app.*/)
      .thenReply(200, ApplicationOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.application.create({ name: "test" }, { idempotencyKey: "random-key" });
    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(requests[0].headers["idempotency-key"]).toBe("random-key");
  });

  test("test retry for status => 500", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app")
      .thenReply(500, `{"code":"500","detail":"asd"}`);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await expect(svx.application.list()).rejects.toThrow(ApiException);

    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(3);

    // same svix-req-id for each retry
    const req_id = requests[0].headers["svix-req-id"];
    expect(req_id).toBeDefined();
    expect(typeof req_id).toBe("string");
    for (let i = 0; i < requests.length; i++) {
      expect(requests[i].headers["svix-req-id"]).toBe(req_id);
      if (i == 0) {
        // first request does not set svix-retry-count
        expect(requests[i].headers["svix-retry-count"]).toBeUndefined();
      } else {
        expect(requests[i].headers["svix-retry-count"]).toBe(i.toString());
      }
    }
  });

  test("no body in response does not return anything", async () => {
    const endpointMock = await mockServer
      .forDelete("/api/v1/app/app1")
      .thenReply(204, "");
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.application.delete("app1");

    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
  });

  test("422 returns validation error", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app")
      .thenReply(422, ValidationErrorOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await expect(svx.application.list()).rejects.toThrow(ApiException<ValidationError>);
    try {
      await svx.application.list();
    } catch (e) {
      expect(e).toHaveProperty("code", 422);
      expect(e).toHaveProperty("body", {
        detail: [{ loc: ["string"], msg: "string", type: "string" }],
      });
    }

    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(2);
  });

  test("400 returns ApiException", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app")
      .thenReply(400, `{"code":"400","detail":"text"}`);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await expect(svx.application.list()).rejects.toThrow(ApiException<HttpErrorOut>);
    try {
      await svx.application.list();
    } catch (e) {
      expect(e).toHaveProperty("code", 400);
      expect(e).toHaveProperty("body", { code: "400", detail: "text" });
    }

    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(2);
  });

  test("sub-resource works", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/operational-webhook/endpoint")
      .thenReply(200, ListResponseOperationalWebhookEndpointOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.operationalWebhookEndpoint.list();

    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
  });

  test("integer enum serialization", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app/app1/attempt/endpoint/endp1")
      .thenReply(200, ListResponseMessageAttemptOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const res = await svx.messageAttempt.listByEndpoint("app1", "endp1");
    expect(res.data[0].triggerType).toBe(MessageAttemptTriggerType.Scheduled);

    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
  });

  test("string enum de/serialization", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/background-task")
      .thenReply(200, ListResponseBackgroundTaskOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const res = await svx.backgroundTask.list({
      task: BackgroundTaskType.EndpointReplay,
    });
    expect(res.data[0].task).toBe(BackgroundTaskType.EndpointReplay);

    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(requests[0].url.endsWith("api/v1/background-task?task=endpoint.replay")).toBe(
      true
    );
  });

  test("non-camelCase field name", async () => {
    const endpointMock = await mockServer
      .forPost("/api/v1/event-type/import/openapi")
      .thenReply(200, EventTypeImportOpenApiOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const res = await svx.eventType.importOpenapi({ dryRun: true });
    expect(res.data.toModify).toStrictEqual([
      {
        name: "user.signup",
        description: "string",
        schemas: {},
        deprecated: true,
        featureFlag: "cool-new-feature",
        groupName: "user",
      },
    ]);

    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
  });

  test("patch request body", async () => {
    const endpointMock = await mockServer
      .forPatch("/api/v1/app/app1/endpoint/endp1")
      .thenReply(200, EndpointOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.endpoint.patch("app1", "endp1", { filterTypes: ["ty1"] });
    await svx.endpoint.patch("app1", "endp1", { filterTypes: null });
    await svx.endpoint.patch("app1", "endp1", { description: "text" });

    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(3);

    // nullable field is sent
    expect(await requests[0].body.getText()).toBe(`{"filterTypes":["ty1"]}`);
    // nullable field is null
    expect(await requests[1].body.getText()).toBe(`{"filterTypes":null}`);
    // undefined field is omitted
    expect(await requests[2].body.getText()).toBe(`{"description":"text"}`);
  });

  test("arbitrary json object body", async () => {
    const endpointMock = await mockServer
      .forPost("/api/v1/app/app1/msg")
      .thenReply(200, EndpointOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.message.create("app1", {
      eventType: "asd",
      payload: { key1: "val", list: ["val1"], obj: { key: "val2" } },
    });

    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(await requests[0].body.getText()).toBe(
      `{"eventType":"asd","payload":{"key1":"val","list":["val1"],"obj":{"key":"val2"}}}`
    );
  });

  test("token/user-agent is sent", async () => {
    const endpointMock = await mockServer
      .forDelete("/api/v1/app/app1")
      .thenReply(200, EndpointOut);
    const svx = new Svix("token.eu", { serverUrl: mockServer.url });

    await svx.application.delete("app1");

    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(requests[0].headers["authorization"]).toBe("Bearer token.eu");
    expect(requests[0].headers["user-agent"]).toBe(`svix-libs/${LIB_VERSION}/javascript`);
  });

  test("MessageAttemptOut without msg", async () => {
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
    expect(requests.length).toBe(1);
  });

  test("octothorpe in url query", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app/app1/msg")
      .thenReply(200, ListResponseMessageOut);
    const svx = new Svix("token.eu", { serverUrl: mockServer.url });

    await svx.message.list("app1", { tag: "test#test" });

    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(requests[0].url.endsWith("api/v1/app/app1/msg?tag=test%23test")).toBe(true);
  });

  test("content-type application/json is sent on request with body", async () => {
    const endpointMock = await mockServer
      .forPost(/\/api\/v1\/app.*/)
      .thenReply(200, ApplicationOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.application.create({ name: "test" });
    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(requests[0].headers["content-type"]).toBe("application/json");
  });

  test("content type not sent on request without body", async () => {
    const endpointMock = await mockServer
      .forGet("/api/v1/app")
      .thenReply(200, ListResponseApplicationOut);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    await svx.application.list();
    const requests = await endpointMock.getSeenRequests();
    expect(requests.length).toBe(1);
    expect(requests[0].headers["content-type"]).toBeUndefined();
  });

  test("struct enum with extra fields", async () => {
    const endpointMock = await mockServer
      .forPost("/ingest/api/v1/source")
      .thenReply(200, IngestSourceOutCron);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const res = await svx.ingest.source.create({
      name: "crontab -r",
      type: "cron",
      config: { schedule: "hello", payload: "world" },
    });

    expect(res.type).toBe("cron");
    // this will smart cast res.config
    if (res.type === "cron") {
      expect(res.config.schedule).toBe("hello");
      expect(res.config.payload).toBe("world");
    }
    const requests = await endpointMock.getSeenRequests();
    expect(await requests[0].body.getText()).toBe(
      '{"type":"cron","config":{"payload":"world","schedule":"hello"},"name":"crontab -r"}'
    );
  });

  test("struct enum with no extra fields", async () => {
    const endpointMock = await mockServer
      .forPost("/ingest/api/v1/source")
      .thenReply(200, IngestSourceOutGeneric);
    const svx = new Svix("token", { serverUrl: mockServer.url });

    const res = await svx.ingest.source.create({
      name: "generic over <T>",
      type: "generic-webhook",
    });

    expect(res.type).toBe("generic-webhook");
    const requests = await endpointMock.getSeenRequests();
    // empty config object should be sent
    expect(await requests[0].body.getText()).toBe(
      '{"type":"generic-webhook","config":{},"name":"generic over <T>"}'
    );
  });
});
