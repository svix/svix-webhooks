import { Svix } from ".";
import type { HttpErrorOut } from "./HttpErrors";
import type { ApiException } from "./util";
import { test } from "node:test";
import { strict as assert } from "node:assert/strict";

function getTestClient(): Svix | null {
  const token = process.env.SVIX_TOKEN;
  const serverUrl = process.env.SVIX_SERVER_URL;
  if (!token || !serverUrl) {
    console.warn(
      "Unable to construct test client: `SVIX_TOKEN` or `SVIX_SERVER_URL` unset."
    );
    return null;
  }
  return new Svix(token, { serverUrl });
}
const client = getTestClient();

// Auto-skip tests in this module if we don't have a test client to work with.
test("e2e tests", { skip: client === null }, async (t) => {
  await t.test("endpoint crud", async () => {
    if (client === null || client === undefined) {
      throw new Error("unreachable");
    }

    const appOut = await client.application.create({ name: "App" });
    try {
      await client.eventType.create({
        name: "event.started",
        description: "Something started",
      });
    } catch (e) {
      // Conflicts are expected from test run to test run, but other statuses are not.
      assert.deepEqual((e as ApiException<HttpErrorOut>).code, 409);
    }
    try {
      await client.eventType.create({
        name: "event.ended",
        description: "Something ended",
      });
    } catch (e) {
      // Conflicts are expected from test run to test run, but other statuses are not.
      assert.deepEqual((e as ApiException<HttpErrorOut>).code, 409);
    }

    const epOut = await client.endpoint.create(appOut.id, {
      url: "https://example.svix.com/",
      channels: ["ch0", "ch1"],
    });
    assert.deepEqual(epOut.channels?.sort(), ["ch0", "ch1"]);
    assert.deepEqual(epOut.filterTypes || [], []);

    const epPatched = await client.endpoint.patch(appOut.id, epOut.id, {
      filterTypes: ["event.started", "event.ended"],
    });

    assert.deepEqual(epPatched.channels?.sort(), ["ch0", "ch1"]);
    assert.deepEqual(epPatched.filterTypes?.sort(), ["event.ended", "event.started"]);

    // Should not throw an error while trying to deserialize the empty body.
    await client.endpoint.delete(appOut.id, epOut.id);
  });
});
