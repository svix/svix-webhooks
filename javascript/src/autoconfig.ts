import { SvixInternal } from "./api_internal";
import { Endpoint as InternalEndpoint } from "./api_internal/endpoint";
import type { EndpointIn } from "./models/endpointIn";
import type { EndpointOut } from "./models/endpointOut";
import type { SvixRequestContext } from "./request";
import {
  Webhook,
  type WebhookRequiredHeaders,
  type WebhookUnbrandedRequiredHeaders,
} from "./webhook";

const AUTOCONFIG_TOKEN_PREFIX_V1 = "auto_v1_";
const AUTOCONFIG_TOKEN_PREFIX_V2 = "auto_v2_";

const UNSUPPORTED_TOKEN_VERSION =
  "Unsupported token version. You might need to update the Svix SDK to use this token";

export interface AutoConfigTokenContentV1 {
  aid: string;
  eid: string;
  surl: string;
  esec: string;
  tok: string;
}

export interface AutoConfigTokenContentV2 {
  aid: string;
  sid: string;
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

export function isAutoConfigTokenContentV1(
  value: unknown
): value is AutoConfigTokenContentV1 {
  if (typeof value !== "object" || value === null) {
    return false;
  }

  const { aid, eid, surl, esec, tok } = value as AutoConfigTokenContentV1;
  return (
    typeof aid === "string" &&
    typeof eid === "string" &&
    typeof surl === "string" &&
    typeof esec === "string" &&
    typeof tok === "string"
  );
}

function isAutoConfigTokenContentV2(value: unknown): value is AutoConfigTokenContentV2 {
  if (typeof value !== "object" || value === null) {
    return false;
  }
  const { aid, sid, surl, esec, tok } = value as AutoConfigTokenContentV2;
  return (
    typeof aid === "string" &&
    typeof sid === "string" &&
    typeof surl === "string" &&
    typeof esec === "string" &&
    typeof tok === "string"
  );
}

function parseTokenPayload(token: string, prefix: string): unknown {
  if (!token.startsWith(prefix)) {
    throw new AutoConfigError(UNSUPPORTED_TOKEN_VERSION);
  }
  try {
    return JSON.parse(Buffer.from(token.slice(prefix.length), "base64").toString("utf8"));
  } catch {
    throw new AutoConfigError();
  }
}

export function decodeAutoconfigTokenV1(token: string): AutoConfigTokenContentV1 {
  const parsed = parseTokenPayload(token, AUTOCONFIG_TOKEN_PREFIX_V1);
  if (!isAutoConfigTokenContentV1(parsed)) {
    throw new AutoConfigError();
  }
  return parsed;
}

export function decodeAutoconfigTokenV2(token: string): AutoConfigTokenContentV2 {
  const parsed = parseTokenPayload(token, AUTOCONFIG_TOKEN_PREFIX_V2);
  if (!isAutoConfigTokenContentV2(parsed)) {
    throw new AutoConfigError();
  }
  return parsed;
}

export function decodeAutoconfigToken(
  token: string
):
  | { version: "v1"; content: AutoConfigTokenContentV1 }
  | { version: "v2"; content: AutoConfigTokenContentV2 } {
  if (token.startsWith(AUTOCONFIG_TOKEN_PREFIX_V1)) {
    return { version: "v1", content: decodeAutoconfigTokenV1(token) };
  }
  if (token.startsWith(AUTOCONFIG_TOKEN_PREFIX_V2)) {
    return { version: "v2", content: decodeAutoconfigTokenV2(token) };
  }
  throw new AutoConfigError(UNSUPPORTED_TOKEN_VERSION);
}

export class AutoConfig {
  private readonly appId: string;
  private readonly webhook: Webhook;
  private readonly requestCtx: SvixRequestContext;
  private readonly endpointIn: EndpointIn;
  private readonly endpointId?: string;
  private readonly autoconfigId?: string;

  public constructor(token: string, endpoint: EndpointIn) {
    const decoded = decodeAutoconfigToken(token);

    let webhook: Webhook;
    try {
      webhook = new Webhook(decoded.content.esec);
    } catch {
      throw new AutoConfigError();
    }

    this.appId = decoded.content.aid;
    this.endpointIn = endpoint;
    this.webhook = webhook;

    if (decoded.version === "v1") {
      this.endpointId = decoded.content.eid;
    } else {
      this.autoconfigId = decoded.content.sid;
    }

    const svix = new SvixInternal(decoded.content.tok, {
      serverUrl: decoded.content.surl,
    });
    this.requestCtx = svix.getRequestCtx();
  }

  public subscribe(): Promise<EndpointOut> {
    const endpoint = new InternalEndpoint(this.requestCtx);
    if (this.autoconfigId != null) {
      return endpoint.autoconfig.subscribe(
        this.appId,
        this.autoconfigId,
        this.endpointIn
      );
    }
    return endpoint.autoConfigDeprecated.update(this.appId, this.endpointId as string, {
      endpoint: this.endpointIn,
    });
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
