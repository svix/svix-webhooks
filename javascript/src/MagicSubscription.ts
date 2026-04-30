import { Webhook, WebhookOptions, WebhookRequiredHeaders, WebhookUnbrandedRequiredHeaders } from "./webhook";
import { EndpointOut, EndpointUpdate } from "./models";
import { Svix } from ".";

export type MagicSubscriptionOptions = EndpointUpdate & { key: string, verifyOptions?: WebhookOptions };

interface MagicTokenPayloadShort {
  // Endpoint ID
  i: string;
  // Server URL
  u: string;
  // Endpoint Secret
  s: string;
  // Token
  t: string;
}

interface MagicTokenPayload {
  endpointId: string;
  token: string;
  serverUrl: string;
  endpointSecret: string;
}

export class MagicSubscription {
  private readonly webhook: Webhook;
  private readonly svix: Svix;
  private readonly endpointUpdate: EndpointUpdate
  private readonly endpointId: string;

  constructor(options: MagicSubscriptionOptions) {
    const { key, verifyOptions, ...endpointUpdate } = options;
    const { endpointId, token, serverUrl, endpointSecret } = this.parseMagicToken(key);
    this.webhook = new Webhook(endpointSecret, verifyOptions);
    this.svix = new Svix(token, {
      serverUrl,
    });
    this.endpointId = endpointId;
    this.endpointUpdate = endpointUpdate;
  }

  private parseMagicToken(key: string): MagicTokenPayload {
    try {
      const payload: MagicTokenPayloadShort = JSON.parse(Buffer.from(key, 'base64').toString('utf-8'));
      return {
        endpointId: payload.i,
        token: payload.t,
        serverUrl: payload.u,
        endpointSecret: payload.s,
      };
    } catch (error) {
      throw new Error("Invalid magic token");
    }
  }

  public subscribe(): Promise<EndpointOut> {
    // FIXME: use the magic subscription API call
    return this.svix.endpoint.update("<APP_ID>", this.endpointId, this.endpointUpdate);
  }

  public verify(
    payload: string | Buffer,
    headers:
      | WebhookRequiredHeaders
      | WebhookUnbrandedRequiredHeaders
      | Record<string, string>
  ): unknown {
    return this.webhook.verify(payload, headers);
  }

  public sign(msgId: string, timestamp: Date, payload: string | Buffer): string {
    return this.webhook.sign(msgId, timestamp, payload);
  }
}
