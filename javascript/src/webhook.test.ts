import { expect, test } from "vitest";
import { fromUint8Array, toUint8Array } from "js-base64";

import { Webhook, WebhookVerificationError } from "./index";

const textEncoder = new globalThis.TextEncoder();

const defaultMsgID = "msg_p5jXN8AQM9LWM0D4loKWxJek";
const defaultPayload = `{"test": 2432232314}`;
const defaultSecret = "MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw";

const tolerance_in_ms = 5 * 60 * 1000;

async function TestPayload(timestamp = Date.now()): Promise<{
  id: string;
  timestamp: number;
  header: Record<string, string>;
  payload: string;
  signature: string;
}> {
  const id = defaultMsgID;
  timestamp = Math.floor(timestamp / 1000);
  const payload = defaultPayload;

  const key = await crypto.subtle.importKey(
    "raw",
    toUint8Array(defaultSecret),
    { name: "HMAC", hash: "SHA-256" },
    false,
    ["sign"]
  );

  const toSign = textEncoder.encode(`${id}.${timestamp}.${payload}`);

  const signature = fromUint8Array(
    new Uint8Array(await globalThis.crypto.subtle.sign("HMAC", key, toSign))
  );
  const header = {
    "svix-id": id,
    "svix-signature": "v1," + signature,
    "svix-timestamp": timestamp.toString(),
  };

  return {
    id,
    timestamp,
    header,
    payload,
    signature,
  };
}

test("empty key raises error", async () => {
  expect(() => {
    new Webhook("");
  }).toThrowError(Error);
  expect(() => {
    new Webhook(undefined as any);
  }).toThrowError(Error);
  expect(() => {
    new Webhook(null as any);
  }).toThrowError(Error);
});

test("missing id raises error", async () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = await TestPayload();
  delete testPayload.header["svix-id"];

  expect(async () => {
    await wh.verify(testPayload.payload, testPayload.header);
  }).rejects.toThrow(WebhookVerificationError);
});

test("missing timestamp raises error", async () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = await TestPayload();
  delete testPayload.header["svix-timestamp"];

  expect(async () => {
    await wh.verify(testPayload.payload, testPayload.header);
  }).rejects.toThrow(WebhookVerificationError);
});

test("invalid timestamp throws error", async () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = await TestPayload();
  testPayload.header["svix-timestamp"] = "hello";

  expect(async () => {
    await wh.verify(testPayload.payload, testPayload.header);
  }).rejects.toThrow(WebhookVerificationError);
});

test("missing signature raises error", async () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = await TestPayload();
  delete testPayload.header["svix-signature"];

  expect(async () => {
    await wh.verify(testPayload.payload, testPayload.header);
  }).rejects.toThrow(WebhookVerificationError);
});

test("invalid signature throws error", async () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = await TestPayload();
  testPayload.header["svix-signature"] = "v1,dawfeoifkpqwoekfpqoekf";

  expect(async () => {
    await wh.verify(testPayload.payload, testPayload.header);
  }).rejects.toThrow(WebhookVerificationError);
});

test("valid signature is valid and returns valid json", async () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = await TestPayload();

  await wh.verify(testPayload.payload, testPayload.header);
});

test("valid unbranded signature is valid and returns valid json", async () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = await TestPayload();
  const unbrandedHeaders: Record<string, string> = {
    "webhook-id": testPayload.header["svix-id"],
    "webhook-signature": testPayload.header["svix-signature"],
    "webhook-timestamp": testPayload.header["svix-timestamp"],
  };
  testPayload.header = unbrandedHeaders;

  await wh.verify(testPayload.payload, testPayload.header);
});

test("old timestamp fails", async () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = await TestPayload(Date.now() - tolerance_in_ms - 1000);

  expect(async () => {
    await wh.verify(testPayload.payload, testPayload.header);
  }).rejects.toThrow(WebhookVerificationError);
});

test("new timestamp fails", async () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = await TestPayload(Date.now() + tolerance_in_ms + 1000);

  expect(async () => {
    await wh.verify(testPayload.payload, testPayload.header);
  }).rejects.toThrow(WebhookVerificationError);
});

test("multi sig payload is valid", async () => {
  const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

  const testPayload = await TestPayload();
  const sigs = [
    "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
    "v2,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
    testPayload.header["svix-signature"], // valid signature
    "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
  ];
  testPayload.header["svix-signature"] = sigs.join(" ");

  await wh.verify(testPayload.payload, testPayload.header);
});

test("verification works with and without signature prefix", async () => {
  const testPayload = await TestPayload();

  let wh = new Webhook(defaultSecret);
  await wh.verify(testPayload.payload, testPayload.header);

  wh = new Webhook("whsec_" + defaultSecret);
  await wh.verify(testPayload.payload, testPayload.header);
});

test("sign function works", async () => {
  const key = "whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw";
  const msgId = "msg_p5jXN8AQM9LWM0D4loKWxJek";
  const timestamp = new Date(1614265330 * 1000);
  const payload = '{"test": 2432232314}';
  const expected = "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE=";

  const wh = new Webhook(key);

  const signature = await wh.sign(msgId, timestamp, payload);
  expect(signature).toBe(expected);
});
