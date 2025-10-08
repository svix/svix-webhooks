// this file is @generated
import { type EventIn, EventInSerializer } from "./eventIn";
import { type StreamIn, StreamInSerializer } from "./streamIn";

export interface CreateStreamEventsIn {
  events: EventIn[];
  /**
   * Optionally creates a new Stream alongside the events.
   *
   * If the stream id or uid that is used in the path already exists, this argument is ignored.
   */
  stream?: StreamIn | null;
}

export const CreateStreamEventsInSerializer = {
  _fromJsonObject(object: any): CreateStreamEventsIn {
    return {
      events: object["events"].map((item: EventIn) =>
        EventInSerializer._fromJsonObject(item)
      ),
      stream: object["stream"]
        ? StreamInSerializer._fromJsonObject(object["stream"])
        : undefined,
    };
  },

  _toJsonObject(self: CreateStreamEventsIn): any {
    return {
      events: self.events.map((item) => EventInSerializer._toJsonObject(item)),
      stream: self.stream ? StreamInSerializer._toJsonObject(self.stream) : undefined,
    };
  },
};
