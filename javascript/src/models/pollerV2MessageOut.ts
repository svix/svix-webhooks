// this file is @generated

/** The MessageOut equivalent of polling endpoint */
export interface PollerV2MessageOut {
  offset: number;
  headers?: { [key: string]: string } | null;
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

export const PollerV2MessageOutSerializer = {
  _fromJsonObject(object: any): PollerV2MessageOut {
    return {
      offset: object["offset"],
      headers: object["headers"],
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

  _toJsonObject(self: PollerV2MessageOut): any {
    return {
      offset: self.offset,
      headers: self.headers,
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
