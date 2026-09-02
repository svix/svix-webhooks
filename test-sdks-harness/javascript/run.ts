import { readFileSync } from "node:fs";
import { AutoConfig, AutoConfigConsumer } from "../../javascript/src/index.ts";
import { StartingPosition } from "../../javascript/src/models/startingPosition.ts";

const fixturesPath = process.env.AUTOCONFIG_FIXTURES;
if (!fixturesPath) {
  throw new Error("AUTOCONFIG_FIXTURES is unset");
}

const fixtures = JSON.parse(readFileSync(fixturesPath, "utf8"));
const lang = fixtures.languages.javascript;
const httpUrl: string = fixtures.httpUrl;
const eventType: string = fixtures.eventType;
const consumerId: string = fixtures.consumerId;
const serverUrl: string = fixtures.serverUrl;
const orgToken: string = fixtures.orgToken;
const appId: string = lang.appId;

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function errMsg(e: unknown): string {
  if (e instanceof Error) {
    const extra = (e as { body?: unknown }).body;
    return extra != null ? `${e.message} ${JSON.stringify(extra)}` : e.message;
  }
  return String(e);
}

async function postMessage(src: string): Promise<void> {
  const resp = await fetch(`${serverUrl}/api/v1/app/${appId}/msg/`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${orgToken}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ eventType, payload: { ok: true, src } }),
  });
  const text = await resp.text();
  if (resp.status < 200 || resp.status >= 300) {
    throw new Error(`send msg HTTP ${resp.status}: ${text}`);
  }
}

async function testHttp(label: string, token: string): Promise<string> {
  try {
    const out = await new AutoConfig(token, {
      url: httpUrl,
      eventTypes: [eventType],
    }).subscribe();
    if (out?.url === httpUrl && out.id) {
      return `PASS — ${label} EndpointOut.url=${out.url} id=${out.id}`;
    }
    return `FAIL — ${label} url/id mismatch got url=${out?.url} id=${out?.id}`;
  } catch (e) {
    return `FAIL — ${label} ${errMsg(e)}`;
  }
}

async function receiveUntil(
  consumer: AutoConfigConsumer,
  src: string,
): Promise<{ offset: number } | { error: string }> {
  let last = "";
  for (let i = 0; i < 10; i++) {
    const poll = await consumer.receive(consumerId, {
      startingPosition: StartingPosition.Earliest,
      leaseDurationMs: 2000,
    });
    last = `attempt=${i + 1} n=${poll.data.length}`;
    const match = poll.data.find(
      (m) => m.eventType === eventType && m.payload && m.payload.src === src,
    );
    if (match) {
      return { offset: match.offset };
    }
    await sleep(2000);
  }
  return { error: `no matching message after 10 receives (${last})` };
}

async function testPollerExisting(
  label: string,
  token: string,
  src: string,
): Promise<string> {
  try {
    const binder = new AutoConfigConsumer(token, {
      eventTypes: [eventType],
    });
    const dest = await binder.subscribe();
    if (!dest?.id) {
      return `FAIL — ${label} subscribe missing DestinationOut.id`;
    }
    const consumer = new AutoConfigConsumer(token, {
      eventTypes: [eventType],
    });
    await postMessage(src);
    const got = await receiveUntil(consumer, src);
    if ("error" in got) {
      return `FAIL — ${label} dest=${dest.id} ${got.error}`;
    }
    await consumer.commit(consumerId, got.offset);
    const again = await consumer.receive(consumerId, {
      startingPosition: StartingPosition.Earliest,
      leaseDurationMs: 2000,
    });
    if (again.data.some((m) => m.offset === got.offset)) {
      return `FAIL — ${label} dest=${dest.id} offset ${got.offset} replayed after commit`;
    }
    return `PASS — ${label} dest=${dest.id} offset=${got.offset} not replayed (no subscribe on receiver)`;
  } catch (e) {
    return `FAIL — ${label} ${errMsg(e)}`;
  }
}

async function testPoller(label: string, token: string, src: string): Promise<string> {
  try {
    const consumer = new AutoConfigConsumer(token, {
      eventTypes: [eventType],
    });
    const dest = await consumer.subscribe();
    if (!dest?.id) {
      return `FAIL — ${label} subscribe missing DestinationOut.id`;
    }
    await postMessage(src);
    const got = await receiveUntil(consumer, src);
    if ("error" in got) {
      return `FAIL — ${label} dest=${dest.id} ${got.error}`;
    }
    await consumer.commit(consumerId, got.offset);
    const again = await consumer.receive(consumerId, {
      startingPosition: StartingPosition.Earliest,
      leaseDurationMs: 2000,
    });
    if (again.data.some((m) => m.offset === got.offset)) {
      return `FAIL — ${label} dest=${dest.id} offset ${got.offset} replayed after commit`;
    }
    return `PASS — ${label} dest=${dest.id} offset=${got.offset} not replayed`;
  } catch (e) {
    return `FAIL — ${label} ${errMsg(e)}`;
  }
}

async function main() {
  const a = await testHttp("v1 HTTP", lang.v1Http);
  const b = await testHttp("v2 HTTP", lang.v2Http);
  const c = await testPoller("v1 poller", lang.v1Poller, "javascript-v1");
  const d = await testPoller("v2 poller", lang.v2Poller, "javascript-v2");
  const e = await testPollerExisting(
    "v2 receive without subscribe",
    lang.v2PollerExisting,
    "javascript-e",
  );
  console.log("LANG: javascript");
  console.log(`A: ${a}`);
  console.log(`B: ${b}`);
  console.log(`C: ${c}`);
  console.log(`D: ${d}`);
  console.log(`E: ${e}`);
  console.log(`Notes: appId=${appId}; imported javascript/src`);
}

main().catch((e) => {
  console.log("LANG: javascript");
  console.log(`A: FAIL — uncaught ${errMsg(e)}`);
  console.log(`B: FAIL — uncaught ${errMsg(e)}`);
  console.log(`C: FAIL — uncaught ${errMsg(e)}`);
  console.log(`D: FAIL — uncaught ${errMsg(e)}`);
  console.log(`E: FAIL — uncaught ${errMsg(e)}`);
  console.log(`Notes: uncaught ${errMsg(e)}`);
  process.exit(1);
});
