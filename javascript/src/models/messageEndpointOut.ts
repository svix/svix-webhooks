// this file is @generated
import { type MessageStatus, MessageStatusSerializer } from "./messageStatus";
import { type MessageStatusText, MessageStatusTextSerializer } from "./messageStatusText";

export interface MessageEndpointOut {
  /** The Endpoint's ID. */
  id: string;
  status: MessageStatus;
  statusText: MessageStatusText;
  nextAttempt?: Date | null;
  /** An example endpoint name. */
  description: string;
  /**
   * Maximum messages per second to send to this endpoint.
   *
   * Outgoing messages will be throttled to this rate.
   */
  throttleRate?: number | null;
  /** Optional unique identifier for the endpoint. */
  uid?: string | null;
  url: string;
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
      description: object["description"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      url: object["url"],
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
      description: self.description,
      throttleRate: self.throttleRate,
      uid: self.uid,
      url: self.url,
      disabled: self.disabled,
      eventTypes: self.eventTypes,
      channels: self.channels,
      createdAt: self.createdAt,
      updatedAt: self.updatedAt,
    };
  },
};
