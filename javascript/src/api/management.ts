import { SvixRequestContext } from "../request";
import { ManagementAuthentication } from "./managementAuthentication";

export class Management {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  public get authentication() {
    return new ManagementAuthentication(this.requestCtx);
  }
}
