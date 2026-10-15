import { ApiException, Svix } from "svix";
import { env } from "./_generated/server.js";

// Must only be called from within a handler, as `env` isn't available while
// the module is being analyzed.
export function svixClient(): Svix {
  return new Svix(env.SVIX_API_KEY, {
    serverUrl: env.SVIX_SERVER_URL,
    // Delivery is retried by the workpool, which tracks each attempt.
    numRetries: 0,
  });
}

/** Whether a failed request to Svix is worth retrying. */
export function isRetryable(error: unknown): boolean {
  if (error instanceof ApiException) {
    return error.code >= 500 || error.code === 429;
  }
  // Network errors, timeouts, etc.
  return true;
}

/** Whether Svix rejected a message because its `eventId` was already used. */
export function isConflict(error: unknown): boolean {
  return error instanceof ApiException && error.code === 409;
}

export function errorMessage(error: unknown): string {
  if (error instanceof ApiException) {
    return `Svix API error ${error.code}: ${JSON.stringify(error.body)}`;
  }
  return error instanceof Error ? error.message : String(error);
}
