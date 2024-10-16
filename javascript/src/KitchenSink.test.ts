import {ApplicationIn, Svix} from ".";

function getTestClient() : Svix | null {
    const token = process.env.SVIX_TOKEN;
    const serverUrl = process.env.SVIX_SERVER_URL;
    if (!token || !serverUrl) {
        console.warn("Unable to construct test client: `SVIX_TOKEN` or `SVIX_SERVER_URL` unset.")
        return null;
    }
    return new Svix(token, { serverUrl });
}
const client = getTestClient();

// Auto-skip tests in this module if we don't have a test client to work with.
test = client == null ? test.skip : test;

test("endpoint crud", async () => {
  const appOut = await client?.application.create({ name: "App"});
  expect(appOut?.id).toBe("xxx");
});
