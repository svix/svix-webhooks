// this file is @generated

export interface MessageOut {
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

export const MessageOutSerializer = {
  _fromJsonObject(object: any): MessageOut {
    return {
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

  _toJsonObject(self: MessageOut): any {
    return {
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
