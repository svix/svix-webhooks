// this file is @generated

/** The MessageOut equivalent of polling endpoint */
export interface PollerV2MessageOut {
  /** List of free-form identifiers that endpoints can filter by */
  channels?: string[] | null;
  deliverAt?: Date | null;
  /** Optional unique identifier for the message */
  eventId?: string | null;
  /** The event type's name */
  eventType: string;
  headers?: { [key: string]: string } | null;
  /** The Message's ID. */
  id: string;
  offset: number;
  payload: any;
  tags?: string[] | null;
  timestamp: Date;
}

export const PollerV2MessageOutSerializer = {
  _fromJsonObject(object: any): PollerV2MessageOut {
    return {
      channels: object["channels"],
      deliverAt: object["deliverAt"] ? new Date(object["deliverAt"]) : null,
      eventId: object["eventId"],
      eventType: object["eventType"],
      headers: object["headers"],
      id: object["id"],
      offset: object["offset"],
      payload: object["payload"],
      tags: object["tags"],
      timestamp: new Date(object["timestamp"]),
    };
  },

  _toJsonObject(self: PollerV2MessageOut): any {
    return {
      channels: self.channels,
      deliverAt: self.deliverAt,
      eventId: self.eventId,
      eventType: self.eventType,
      headers: self.headers,
      id: self.id,
      offset: self.offset,
      payload: self.payload,
      tags: self.tags,
      timestamp: self.timestamp,
    };
  },
};
