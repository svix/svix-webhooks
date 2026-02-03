// this file is @generated
import { type MessageStatus, MessageStatusSerializer } from "./messageStatus";

export interface BulkReplayIn {
  channel?: string | null;
  eventTypes?: string[] | null;
  since: Date;
  status?: MessageStatus | null;
  tag?: string | null;
  until?: Date | null;
}

export const BulkReplayInSerializer = {
  _fromJsonObject(object: any): BulkReplayIn {
    return {
      channel: object["channel"],
      eventTypes: object["eventTypes"],
      since: new Date(object["since"]),
      status:
        object["status"] != null
          ? MessageStatusSerializer._fromJsonObject(object["status"])
          : undefined,
      tag: object["tag"],
      until: object["until"] ? new Date(object["until"]) : null,
    };
  },

  _toJsonObject(self: BulkReplayIn): any {
    return {
      channel: self.channel,
      eventTypes: self.eventTypes,
      since: self.since,
      status:
        self.status != null
          ? MessageStatusSerializer._toJsonObject(self.status)
          : undefined,
      tag: self.tag,
      until: self.until,
    };
  },
};
