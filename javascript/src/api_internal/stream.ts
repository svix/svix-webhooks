import { SvixRequestContext } from "../request";
import { StreamEventType } from "./streamEventType";
import { StreamEvents } from "./streamEvents";
import { StreamStream } from "./streamStream";

export class Stream {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  public get event_type() {
    return new StreamEventType(this.requestCtx);
  }

  public get events() {
    return new StreamEvents(this.requestCtx);
  }

  public get stream() {
    return new StreamStream(this.requestCtx);
  }
}
