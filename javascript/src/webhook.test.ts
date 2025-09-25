import * as utf8 from "@stablelib/utf8";
import * as base64 from "@stablelib/base64";
import * as sha256 from "fast-sha256";
import { test } from "node:test";
import { strict as assert } from "node:assert/strict";

import { Webhook, WebhookVerificationError } from "./index";

const defaultMsgID = "msg_p5jXN8AQM9LWM0D4loKWxJek";
const defaultPayload = `{"test": 2432232314}`;
const defaultSecret = "MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw";

const tolerance_in_ms = 5 * 60 * 1000;

class TestPayload {
  public id: string;
  public timestamp: number;
  public header: Record<string, string>;
  public secret: string;
  public payload: string;
  public signature: string;

  public constructor(timestamp = Date.now()) {
    this.id = defaultMsgID;
    this.timestamp = Math.floor(timestamp / 1000);

    this.payload = defaultPayload;
    this.secret = defaultSecret;

    const toSign = utf8.encode(`${this.id}.${this.timestamp}.${this.payload}`);
    this.signature = base64.encode(sha256.hmac(base64.decode(this.secret), toSign));

    this.header = {
      "svix-id": this.id,
      "svix-signature": `v1,${this.signature}`,
      "svix-timestamp": this.timestamp.toString(),
    };
  }
}

test("empty key raises error", () => {
  assert.throws(() => {
    new Webhook("");
  }, Error);
  assert.throws(() => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    new Webhook(undefined as any);
  }, Error);
  assert.throws(() => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    new Webhook(null as any);
  }, Error);
});

test("missing id raises error", () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = new TestPayload();
  delete testPayload.header["svix-id"];

  assert.throws(() => {
    wh.verify(testPayload.payload, testPayload.header);
  }, WebhookVerificationError);
});

test("missing timestamp raises error", () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = new TestPayload();
  delete testPayload.header["svix-timestamp"];

  assert.throws(() => {
    wh.verify(testPayload.payload, testPayload.header);
  }, WebhookVerificationError);
});

test("invalid timestamp throws error", () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = new TestPayload();
  testPayload.header["svix-timestamp"] = "hello";

  assert.throws(() => {
    wh.verify(testPayload.payload, testPayload.header);
  }, WebhookVerificationError);
});

test("missing signature raises error", () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = new TestPayload();
  delete testPayload.header["svix-signature"];

  assert.throws(() => {
    wh.verify(testPayload.payload, testPayload.header);
  }, WebhookVerificationError);
});

test("invalid signature throws error", () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = new TestPayload();
  testPayload.header["svix-signature"] = "v1,dawfeoifkpqwoekfpqoekf";

  assert.throws(() => {
    wh.verify(testPayload.payload, testPayload.header);
  }, WebhookVerificationError);
});

test("partial signature throws error", () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = new TestPayload();
  testPayload.header["svix-signature"] = testPayload.header["svix-signature"].slice(0, 8);

  assert.throws(() => {
    wh.verify(testPayload.payload, testPayload.header);
  }, WebhookVerificationError);

  testPayload.header["svix-signature"] = "v1,";

  assert.throws(() => {
    wh.verify(testPayload.payload, testPayload.header);
  }, WebhookVerificationError);
});

test("valid signature is valid and returns valid json", () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = new TestPayload();

  wh.verify(testPayload.payload, testPayload.header);
});

test("valid unbranded signature is valid and returns valid json", () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = new TestPayload();
  const unbrandedHeaders: Record<string, string> = {
    "webhook-id": testPayload.header["svix-id"],
    "webhook-signature": testPayload.header["svix-signature"],
    "webhook-timestamp": testPayload.header["svix-timestamp"],
  };
  testPayload.header = unbrandedHeaders;

  wh.verify(testPayload.payload, testPayload.header);
});

test("old timestamp fails", () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = new TestPayload(Date.now() - tolerance_in_ms - 1000);

  assert.throws(() => {
    wh.verify(testPayload.payload, testPayload.header);
  }, WebhookVerificationError);
});

test("new timestamp fails", () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = new TestPayload(Date.now() + tolerance_in_ms + 1000);

  assert.throws(() => {
    wh.verify(testPayload.payload, testPayload.header);
  }, WebhookVerificationError);
});

test("multi sig payload is valid", () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = new TestPayload();
  const sigs = [
    "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
    "v2,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
    testPayload.header["svix-signature"], // valid signature
    "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
  ];
  testPayload.header["svix-signature"] = sigs.join(" ");

  wh.verify(testPayload.payload, testPayload.header);
});

test("verification works with and without signature prefix", () => {
  const testPayload = new TestPayload();

  let wh = new Webhook(defaultSecret);
  wh.verify(testPayload.payload, testPayload.header);

  wh = new Webhook(`whsec_${defaultSecret}`);
  wh.verify(testPayload.payload, testPayload.header);
});

test("sign function works", () => {
  const key = "whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw";
  const msgId = "msg_p5jXN8AQM9LWM0D4loKWxJek";
  const timestamp = new Date(1614265330 * 1000);
  const payload = '{"test": 2432232314}';
  const expected = "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE=";

  const wh = new Webhook(key);

  const signature = wh.sign(msgId, timestamp, payload);
  assert.equal(signature, expected);
});
