// this file is @generated

import { IngestAuthentication } from "./ingestAuthentication";
import { IngestEndpoint } from "./ingestEndpoint";
import { IngestSource } from "./ingestSource";
import type { SvixRequestContext } from "../request";

export class Ingest {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  public get authentication() {
    return new IngestAuthentication(this.requestCtx);
  }

  public get endpoint() {
    return new IngestEndpoint(this.requestCtx);
  }

  public get source() {
    return new IngestSource(this.requestCtx);
  }
}
