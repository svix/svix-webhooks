// this file is @generated

import { OperationalWebhookEndpoint } from "./operationalWebhookEndpoint";
import type { SvixRequestContext } from "../request";

export class OperationalWebhook {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  public get endpoint() {
    return new OperationalWebhookEndpoint(this.requestCtx);
  }
}
