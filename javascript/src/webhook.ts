import { Webhook as StdWh } from "standardwebhooks";
export { WebhookVerificationError } from "standardwebhooks";

export interface WebhookRequiredHeaders {
  "svix-id": string;
  "svix-timestamp": string;
  "svix-signature": string;
}

export interface WebhookUnbrandedRequiredHeaders {
  "webhook-id": string;
  "webhook-timestamp": string;
  "webhook-signature": string;
}

export interface WebhookOptions {
  format?: "raw";
}

export class Webhook {
  private readonly inner: StdWh;

  constructor(secret: string | Uint8Array, options?: WebhookOptions) {
    this.inner = new StdWh(secret, options);
  }

  public verify(
    payload: string | Buffer,
    headers_:
      | WebhookRequiredHeaders
      | WebhookUnbrandedRequiredHeaders
      | Record<string, string>
  ): unknown {
    const headers: Record<string, string> = {};
    for (const key of Object.keys(headers_)) {
      headers[key.toLowerCase()] = (headers_ as Record<string, string>)[key];
    }

    headers["webhook-id"] = headers["svix-id"] ?? headers["webhook-id"] ?? "";
    headers["webhook-signature"] =
      headers["svix-signature"] ?? headers["webhook-signature"] ?? "";
    headers["webhook-timestamp"] =
      headers["svix-timestamp"] ?? headers["webhook-timestamp"] ?? "";

    return this.inner.verify(payload, headers);
  }

  public sign(msgId: string, timestamp: Date, payload: string | Buffer): string {
    return this.inner.sign(msgId, timestamp, payload);
  }
}
