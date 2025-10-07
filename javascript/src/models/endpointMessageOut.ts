// this file is @generated
import { type MessageStatus, MessageStatusSerializer } from "./messageStatus";
import { type MessageStatusText, MessageStatusTextSerializer } from "./messageStatusText";

/** A model containing information on a given message plus additional fields on the last attempt for that message. */
export interface EndpointMessageOut {
  /** List of free-form identifiers that endpoints can filter by */
  channels?: string[] | null;
  deliverAt?: Date | null;
  /** Optional unique identifier for the message */
  eventId?: string | null;
  /** The event type's name */
  eventType: string;
  /** The Message's ID. */
  id: string;
  nextAttempt?: Date | null;
  payload: any;
  status: MessageStatus;
  statusText: MessageStatusText;
  tags?: string[] | null;
  timestamp: Date;
}

export const EndpointMessageOutSerializer = {
  _fromJsonObject(object: any): EndpointMessageOut {
    return {
      channels: object["channels"],
      deliverAt: object["deliverAt"] ? new Date(object["deliverAt"]) : null,
      eventId: object["eventId"],
      eventType: object["eventType"],
      id: object["id"],
      nextAttempt: object["nextAttempt"] ? new Date(object["nextAttempt"]) : null,
      payload: object["payload"],
      status: MessageStatusSerializer._fromJsonObject(object["status"]),
      statusText: MessageStatusTextSerializer._fromJsonObject(object["statusText"]),
      tags: object["tags"],
      timestamp: new Date(object["timestamp"]),
    };
  },

  _toJsonObject(self: EndpointMessageOut): any {
    return {
      channels: self.channels,
      deliverAt: self.deliverAt,
      eventId: self.eventId,
      eventType: self.eventType,
      id: self.id,
      nextAttempt: self.nextAttempt,
      payload: self.payload,
      status: MessageStatusSerializer._toJsonObject(self.status),
      statusText: MessageStatusTextSerializer._toJsonObject(self.statusText),
      tags: self.tags,
      timestamp: self.timestamp,
    };
  },
};
