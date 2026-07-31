import {
  createSvixRequestContext,
  type SvixRequestContext,
  type SvixRequestContextOptions,
} from "../request";

export class SvixInternal {
  private readonly requestCtx: SvixRequestContext;

  public constructor(token: string, options: SvixRequestContextOptions = {}) {
    this.requestCtx = createSvixRequestContext(token, options);
  }

  public getRequestCtx(): SvixRequestContext {
    return this.requestCtx;
  }
}
