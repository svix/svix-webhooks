// this file is @generated

import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export class Health {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Verify the API server is up and running. */
  public async get(): Promise<void> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/health");

    return await request.sendNoResponseBody(this.requestCtx);
  }
}
