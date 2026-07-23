// this file is @generated
import { type MessageStatus, MessageStatusSerializer } from "./messageStatus";
import { type StatusCodeClass, StatusCodeClassSerializer } from "./statusCodeClass";

export interface BulkReplayIn {
  since: Date;
  until?: Date | null;
  eventTypes?: string[] | null;
  channel?: string | null;
  tag?: string | null;
  status?: MessageStatus | null;
  statusCodeClass?: StatusCodeClass | null;
}

export const BulkReplayInSerializer = {
  _fromJsonObject(object: any): BulkReplayIn {
    return {
      since: new Date(object["since"]),
      until: object["until"] ? new Date(object["until"]) : null,
      eventTypes: object["eventTypes"],
      channel: object["channel"],
      tag: object["tag"],
      status:
        object["status"] != null
          ? MessageStatusSerializer._fromJsonObject(object["status"])
          : undefined,
      statusCodeClass:
        object["statusCodeClass"] != null
          ? StatusCodeClassSerializer._fromJsonObject(object["statusCodeClass"])
          : undefined,
    };
  },

  _toJsonObject(self: BulkReplayIn): any {
    return {
      since: self.since,
      until: self.until,
      eventTypes: self.eventTypes,
      channel: self.channel,
      tag: self.tag,
      status:
        self.status != null
          ? MessageStatusSerializer._toJsonObject(self.status)
          : undefined,
      statusCodeClass:
        self.statusCodeClass != null
          ? StatusCodeClassSerializer._toJsonObject(self.statusCodeClass)
          : undefined,
    };
  },
};
