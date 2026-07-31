// this file is @generated
import { type MessageStatus, MessageStatusSerializer } from "./messageStatus";
import { type MessageStatusText, MessageStatusTextSerializer } from "./messageStatusText";

/** A model containing information on a given message plus additional fields on the last attempt for that message. */
export interface EndpointMessageOut {
  status: MessageStatus;
  statusText: MessageStatusText;
  nextAttempt?: Date | null;
  /** Optional unique identifier for the message */
  eventId?: string | null;
  /** The event type's name */
  eventType: string;
  payload: any;
  /** List of free-form identifiers that endpoints can filter by */
  channels?: string[] | null;
  /** The Message's ID. */
  id: string;
  timestamp: Date;
  tags?: string[] | null;
  deliverAt?: Date | null;
}

export const EndpointMessageOutSerializer = {
  _fromJsonObject(object: any): EndpointMessageOut {
    return {
      status: MessageStatusSerializer._fromJsonObject(object["status"]),
      statusText: MessageStatusTextSerializer._fromJsonObject(object["statusText"]),
      nextAttempt: object["nextAttempt"] ? new Date(object["nextAttempt"]) : null,
      eventId: object["eventId"],
      eventType: object["eventType"],
      payload: object["payload"],
      channels: object["channels"],
      id: object["id"],
      timestamp: new Date(object["timestamp"]),
      tags: object["tags"],
      deliverAt: object["deliverAt"] ? new Date(object["deliverAt"]) : null,
    };
  },

  _toJsonObject(self: EndpointMessageOut): any {
    return {
      status: MessageStatusSerializer._toJsonObject(self.status),
      statusText: MessageStatusTextSerializer._toJsonObject(self.statusText),
      nextAttempt: self.nextAttempt,
      eventId: self.eventId,
      eventType: self.eventType,
      payload: self.payload,
      channels: self.channels,
      id: self.id,
      timestamp: self.timestamp,
      tags: self.tags,
      deliverAt: self.deliverAt,
    };
  },
};
