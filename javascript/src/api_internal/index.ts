import { Svix } from "..";
import type { SvixRequestContext } from "../request";

export class SvixInternal extends Svix {
  public getRequestCtx(): SvixRequestContext {
    return this.requestCtx;
  }
}
