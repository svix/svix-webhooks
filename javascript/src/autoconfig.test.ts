import { test } from "node:test";
import { strict as assert } from "node:assert/strict";
import * as mockttp from "mockttp";

import { AutoConfig, AutoConfigConsumer, AutoConfigError, Webhook } from "./index";

function makeToken(
  prefix: "auto_v1_" | "auto_v2_",
  payload: Record<string, string>
): string {
  const json = JSON.stringify(payload);
  return `${prefix}${Buffer.from(json, "utf8").toString("base64")}`;
}

function makeTokenV1(payload: Record<string, string>): string {
  return makeToken("auto_v1_", payload);
}

function makeTokenV2(payload: Record<string, string>): string {
  return makeToken("auto_v2_", payload);
}

const endpointOut = `{"id":"ep_2","metadata":{},"url":"https://consumer.example/webhook","description":"","createdAt":"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z"}`;
const destinationOut = `{"type":"pollingEndpoint","config":{},"id":"dst_123","status":"enabled","currentIterator":"0","createdAt":"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z","batchSize":100,"maxWaitSecs":10,"metadata":{}}`;
const pollOut = `{"data":[],"done":true}`;

test("AutoConfig accepts valid auto_v1 token and verify matches Webhook", () => {
  const esec = "whsec_Zm9v";
  const token = makeTokenV1({
    aid: "app_1",
    eid: "ep_2",
    surl: "https://api.example.test",
    esec,
    tok: "sk_test_xyz",
  });
  const ac = new AutoConfig(token, { url: "https://consumer.example/webhook" });

  const payload = '{"hello":"world"}';
  const id = "msg_test_autoconfig";
  const ts = new Date();
  const wh = new Webhook(esec);
  const sig = wh.sign(id, ts, payload);
  const headers = {
    "svix-id": id,
    "svix-timestamp": Math.floor(ts.getTime() / 1000).toString(),
    "svix-signature": sig,
  };

  ac.verify(payload, headers);
});

test("AutoConfig rejects wrong prefix", () => {
  const json = JSON.stringify({
    aid: "a",
    eid: "e",
    surl: "https://x",
    esec: "whsec_Zm9v",
    tok: "t",
  });
  const token = `wrong_${Buffer.from(json, "utf8").toString("base64")}`;
  assert.throws(
    () => new AutoConfig(token, { url: "https://x" }),
    (err: unknown) =>
      err instanceof AutoConfigError &&
      err.message ===
        "Unsupported token version. You might need to update the Svix SDK to use this token"
  );
});

test("AutoConfig rejects invalid JSON payload", () => {
  const token = `auto_v1_${Buffer.from("not json", "utf8").toString("base64")}`;
  assert.throws(() => new AutoConfig(token, { url: "https://x" }), AutoConfigError);
});

test("AutoConfig accepts valid auto_v2 token and verify matches Webhook", () => {
  const esec = "whsec_Zm9v";
  const token = makeTokenV2({
    aid: "app_1",
    sid: "acfg_2",
    surl: "https://api.example.test",
    esec,
    tok: "sk_test_xyz",
  });
  const ac = new AutoConfig(token, { url: "https://consumer.example/webhook" });

  const payload = '{"hello":"world"}';
  const id = "msg_test_autoconfig_v2";
  const ts = new Date();
  const wh = new Webhook(esec);
  const sig = wh.sign(id, ts, payload);
  const headers = {
    "svix-id": id,
    "svix-timestamp": Math.floor(ts.getTime() / 1000).toString(),
    "svix-signature": sig,
  };

  ac.verify(payload, headers);
});

test("subscribe", async (t) => {
  const mockServer = mockttp.getLocal();
  t.beforeEach(async () => await mockServer.start(0));
  t.afterEach(async () => await mockServer.stop());

  await t.test("v1 AutoConfig", async () => {
    const mock = await mockServer
      .forPut("/api/v1/app/app_1/endpoint/ep_2/auto-config")
      .thenReply(200, endpointOut);
    const token = makeTokenV1({
      aid: "app_1",
      eid: "ep_2",
      surl: mockServer.url,
      esec: "whsec_Zm9v",
      tok: "sk_test_xyz",
    });
    const ac = new AutoConfig(token, { url: "https://consumer.example/webhook" });
    const out = await ac.subscribe();
    assert.equal(out.id, "ep_2");
    const requests = await mock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert.equal(
      await requests[0].body.getText(),
      `{"endpoint":{"url":"https://consumer.example/webhook"}}`
    );
  });

  await t.test("v2 AutoConfig", async () => {
    const mock = await mockServer
      .forPut("/api/v1/app/app_1/autoconfig/acfg_2/endpoint")
      .thenReply(200, endpointOut);
    const token = makeTokenV2({
      aid: "app_1",
      sid: "acfg_2",
      surl: mockServer.url,
      esec: "whsec_Zm9v",
      tok: "sk_test_xyz",
    });
    const ac = new AutoConfig(token, { url: "https://consumer.example/webhook" });
    const out = await ac.subscribe();
    assert.equal(out.id, "ep_2");
    const requests = await mock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert.equal(
      await requests[0].body.getText(),
      `{"url":"https://consumer.example/webhook"}`
    );
  });

  await t.test("v1 AutoConfigConsumer", async () => {
    const mock = await mockServer
      .forPut("/api/v1/app/app_1/endpoint/ep_2/auto-config")
      .thenReply(200, endpointOut);
    const token = makeTokenV1({
      aid: "app_1",
      eid: "ep_2",
      surl: mockServer.url,
      esec: "whsec_Zm9v",
      tok: "sk_test_xyz",
    });
    const consumer = new AutoConfigConsumer(token, {
      type: "pollingEndpoint",
      eventTypes: ["issue.opened"],
    });
    await consumer.subscribe();
    const requests = await mock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert.equal(
      await requests[0].body.getText(),
      `{"sink":{"type":"poller","config":{"eventTypes":["issue.opened"]}}}`
    );
  });

  await t.test("v2 AutoConfigConsumer", async () => {
    const mock = await mockServer
      .forPut("/api/v1/app/app_1/autoconfig/acfg_2/destination")
      .thenReply(200, destinationOut);
    const token = makeTokenV2({
      aid: "app_1",
      sid: "acfg_2",
      surl: mockServer.url,
      esec: "whsec_Zm9v",
      tok: "sk_test_xyz",
    });
    const consumer = new AutoConfigConsumer(token, {
      type: "pollingEndpoint",
      channels: ["ch1"],
    });
    const out = await consumer.subscribe();
    assert.equal(out.id, "dst_123");
    const requests = await mock.getSeenRequests();
    assert.equal(requests.length, 1);
    assert.equal(
      await requests[0].body.getText(),
      `{"type":"pollingEndpoint","config":{},"channels":["ch1"]}`
    );
  });

  await t.test("v2 receive subscribes", async () => {
    await mockServer
      .forPut("/api/v1/app/app_1/autoconfig/acfg_2/destination")
      .thenReply(200, destinationOut);
    const pollMock = await mockServer
      .forGet("/api/v1/app/app_1/polling-endpoint/dst_123/consumer/c1")
      .thenReply(200, pollOut);
    const token = makeTokenV2({
      aid: "app_1",
      sid: "acfg_2",
      surl: mockServer.url,
      esec: "whsec_Zm9v",
      tok: "sk_test_xyz",
    });
    const consumer = new AutoConfigConsumer(token, { type: "pollingEndpoint" });
    await consumer.receive("c1");
    const requests = await pollMock.getSeenRequests();
    assert.equal(requests.length, 1);
  });
});
