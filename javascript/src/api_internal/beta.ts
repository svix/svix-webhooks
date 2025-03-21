import { SvixRequestContext } from "../request";
import { BetaConnector } from "./betaConnector";

export class Beta {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  public get connector() {
    return new BetaConnector(this.requestCtx);
  }
}
