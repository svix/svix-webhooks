import { test } from "node:test";
import { strict as assert } from "node:assert/strict";

import { AutoConfig, AutoConfigError, Webhook } from "./index";

function makeTokenV1(payload: Record<string, string>): string {
  const json = JSON.stringify(payload);
  return `auto_v1_${Buffer.from(json, "utf8").toString("base64")}`;
}

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
  assert.throws(() => new AutoConfig(token, { url: "https://x" }), AutoConfigError);
});

test("AutoConfig rejects invalid JSON payload", () => {
  const token = `auto_v1_${Buffer.from("not json", "utf8").toString("base64")}`;
  assert.throws(() => new AutoConfig(token, { url: "https://x" }), AutoConfigError);
});
