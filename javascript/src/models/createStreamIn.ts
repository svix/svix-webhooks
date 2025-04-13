// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { EventIn, EventInSerializer } from "./eventIn";
import { StreamIn, StreamInSerializer } from "./streamIn";

export interface CreateStreamIn {
  events: EventIn[];
  /**
   * Optionally creates a new Stream alongside the events.
   *
   * If the stream id or uid that is used in the path already exists, this argument is ignored.
   */
  stream?: StreamIn | null;
}

export const CreateStreamInSerializer = {
  _fromJsonObject(object: any): CreateStreamIn {
    return {
      events: object["events"]?.map((item: EventIn) =>
        EventInSerializer._fromJsonObject(item)
      ),
      stream: object["stream"]
        ? StreamInSerializer._fromJsonObject(object["stream"])
        : undefined,
    };
  },

  _toJsonObject(self: CreateStreamIn): any {
    return {
      events: self.events.map((item) => EventInSerializer._toJsonObject(item)),
      stream: self.stream ? StreamInSerializer._toJsonObject(self.stream) : undefined,
    };
  },
};
