// this file is @generated

import { DestinationAutoconfig } from "./destinationAutoconfig";
import type { SvixRequestContext } from "../request";

export class Destination {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  public get autoconfig() {
    return new DestinationAutoconfig(this.requestCtx);
  }
}
