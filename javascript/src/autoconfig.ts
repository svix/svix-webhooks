import { SvixInternal } from "./api_internal";
import { Endpoint as InternalEndpoint } from "./api_internal/endpoint";
import { Svix } from "./index";
import type { EndpointIn } from "./models/endpointIn";
import type { EndpointOut } from "./models/endpointOut";
import type { SvixRequestContext } from "./request";
import {
  Webhook,
  type WebhookRequiredHeaders,
  type WebhookUnbrandedRequiredHeaders,
} from "./webhook";

const AUTOCONFIG_TOKEN_PREFIX_V1 = "auto_v1_";

/** Payload embedded in an {@link AutoConfig} token (`auto_v1_…`), JSON keys match the wire format. */
interface AutoConfigTokenContentV1 {
  aid: string;
  eid: string;
  surl: string;
  esec: string;
  tok: string;
}

export class AutoConfigError extends Error {
  public constructor(message = "invalid token") {
    super(message);
    this.name = "AutoConfigError";
  }
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function decodeAutoconfigTokenV1(token: string): AutoConfigTokenContentV1 {
  if (!token.startsWith(AUTOCONFIG_TOKEN_PREFIX_V1)) {
    throw new AutoConfigError();
  }
  const b64 = token.slice(AUTOCONFIG_TOKEN_PREFIX_V1.length);
  let json: string;
  try {
    json = Buffer.from(b64, "base64").toString("utf8");
  } catch {
    throw new AutoConfigError();
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(json);
  } catch {
    throw new AutoConfigError();
  }
  if (!isRecord(parsed)) {
    throw new AutoConfigError();
  }
  const { aid, eid, surl, esec, tok } = parsed;
  if (
    typeof aid !== "string" ||
    typeof eid !== "string" ||
    typeof surl !== "string" ||
    typeof esec !== "string" ||
    typeof tok !== "string"
  ) {
    throw new AutoConfigError();
  }
  return { aid, eid, surl, esec, tok };
}

export class AutoConfig {
  private readonly appId: string;
  private readonly endpointId: string;
  private readonly endpointIn: EndpointIn;
  private readonly webhook: Webhook;
  private readonly requestCtx: SvixRequestContext;

  public constructor(token: string, endpoint: EndpointIn) {
    const content = decodeAutoconfigTokenV1(token);
    let webhook: Webhook;
    try {
      webhook = new Webhook(content.esec);
    } catch {
      throw new AutoConfigError();
    }

    this.appId = content.aid;
    this.endpointId = content.eid;
    this.endpointIn = endpoint;
    this.webhook = webhook;

    const svix = new SvixInternal(content.tok, { serverUrl: content.surl });
    this.requestCtx = svix.getRequestCtx();
  }

  /** Register this endpoint with Svix using the internal auto-config API. */
  public subscribe(): Promise<EndpointOut> {
    return new InternalEndpoint(this.requestCtx).auto_config.update(
      this.appId,
      this.endpointId,
      {
        endpoint: this.endpointIn,
      }
    );
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
}
