import { SvixRequestContext } from "../request";
import { OperationalWebhookEndpoint } from "./operationalWebhookEndpoint";

export class OperationalWebhook {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  public get endpoint() {
    return new OperationalWebhookEndpoint(this.requestCtx);
  }
}
