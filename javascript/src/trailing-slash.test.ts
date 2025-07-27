import { Svix } from "./index";
import * as mockttp from "mockttp";

const mockServer = mockttp.getLocal();

describe("Trailing Slash Handling", () => {
  beforeEach(async () => await mockServer.start(0));
  afterEach(async () => await mockServer.stop());

  const testEndpoints = [
    "/api/v1/app",
    "/api/v1/app/app_id/endpoint",
    "/ingest/api/v1/source/source_id",
    "/api/v1/operational-webhook/endpoint",
  ];

  test.each(testEndpoints)("should handle trailing slash variations for %s", async () => {
    // Test with trailing slash in base URL
    const clientWithSlash = new Svix("token", { serverUrl: mockServer.url + "/" });

    // Test without trailing slash in base URL
    const clientWithoutSlash = new Svix("token", { serverUrl: mockServer.url });

    // Make requests using internal request context
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const reqCtx1 = (clientWithSlash as any).requestCtx;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const reqCtx2 = (clientWithoutSlash as any).requestCtx;

    // Verify both clients have normalized base URLs (no trailing slash)
    expect(reqCtx1.baseUrl).not.toMatch(/\/$/);
    expect(reqCtx2.baseUrl).not.toMatch(/\/$/);
    expect(reqCtx1.baseUrl).toBe(reqCtx2.baseUrl);
  });

  test("should handle multiple trailing slashes", () => {
    const client = new Svix("token", { serverUrl: "https://api.svix.com///" });
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const reqCtx = (client as any).requestCtx;
    expect(reqCtx.baseUrl).toBe("https://api.svix.com");
  });

  test("should handle single trailing slash", () => {
    const client = new Svix("token", { serverUrl: "https://api.svix.com/" });
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const reqCtx = (client as any).requestCtx;
    expect(reqCtx.baseUrl).toBe("https://api.svix.com");
  });

  test("should preserve URLs without trailing slashes", () => {
    const client = new Svix("token", { serverUrl: "https://api.svix.com" });
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const reqCtx = (client as any).requestCtx;
    expect(reqCtx.baseUrl).toBe("https://api.svix.com");
  });

  test("should handle regional URLs with trailing slashes", () => {
    // Mock a token with EU region
    const client = new Svix("test.eu.token");
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const reqCtx = (client as any).requestCtx;
    expect(reqCtx.baseUrl).toBe("https://api.eu.svix.com");
  });
});
