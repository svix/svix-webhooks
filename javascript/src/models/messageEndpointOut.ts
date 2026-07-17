// this file is @generated
import { type MessageStatus, MessageStatusSerializer } from "./messageStatus";
import { type MessageStatusText, MessageStatusTextSerializer } from "./messageStatusText";

export interface MessageEndpointOut {
  /** The Endpoint's ID. */
  id: string;
  status: MessageStatus;
  statusText: MessageStatusText;
  nextAttempt?: Date | null;
  url: string;
  description: string;
  /**
   * Maximum messages per second to send to this endpoint.
   *
   * Outgoing messages will be throttled to this rate.
   */
  throttleRate?: number | null;
  /** Optional unique identifier for the endpoint. */
  uid?: string | null;
  disabled?: boolean;
  eventTypes?: string[] | null;
  /** List of message channels this endpoint listens to (omit for all). */
  channels?: string[] | null;
  createdAt: Date;
  updatedAt: Date;
}

export const MessageEndpointOutSerializer = {
  _fromJsonObject(object: any): MessageEndpointOut {
    return {
      id: object["id"],
      status: MessageStatusSerializer._fromJsonObject(object["status"]),
      statusText: MessageStatusTextSerializer._fromJsonObject(object["statusText"]),
      nextAttempt: object["nextAttempt"] ? new Date(object["nextAttempt"]) : null,
      url: object["url"],
      description: object["description"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      disabled: object["disabled"],
      eventTypes: object["eventTypes"],
      channels: object["channels"],
      createdAt: new Date(object["createdAt"]),
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: MessageEndpointOut): any {
    return {
      id: self.id,
      status: MessageStatusSerializer._toJsonObject(self.status),
      statusText: MessageStatusTextSerializer._toJsonObject(self.statusText),
      nextAttempt: self.nextAttempt,
      url: self.url,
      description: self.description,
      throttleRate: self.throttleRate,
      uid: self.uid,
      disabled: self.disabled,
      eventTypes: self.eventTypes,
      channels: self.channels,
      createdAt: self.createdAt,
      updatedAt: self.updatedAt,
    };
  },
};
