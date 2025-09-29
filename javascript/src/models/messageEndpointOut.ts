// this file is @generated
import { type MessageStatus, MessageStatusSerializer } from "./messageStatus";
import { type MessageStatusText, MessageStatusTextSerializer } from "./messageStatusText";

export interface MessageEndpointOut {
  /** List of message channels this endpoint listens to (omit for all). */
  channels?: string[] | null;
  createdAt: Date;
  /** An example endpoint name. */
  description: string;
  disabled?: boolean;
  filterTypes?: string[] | null;
  /** The Endpoint's ID. */
  id: string;
  nextAttempt?: Date | null;
  rateLimit?: number | null;
  status: MessageStatus;
  statusText: MessageStatusText;
  /** Optional unique identifier for the endpoint. */
  uid?: string | null;
  updatedAt: Date;
  url: string;
  version: number;
}

export const MessageEndpointOutSerializer = {
  _fromJsonObject(object: any): MessageEndpointOut {
    return {
      channels: object["channels"],
      createdAt: new Date(object["createdAt"]),
      description: object["description"],
      disabled: object["disabled"],
      filterTypes: object["filterTypes"],
      id: object["id"],
      nextAttempt: object["nextAttempt"] ? new Date(object["nextAttempt"]) : null,
      rateLimit: object["rateLimit"],
      status: MessageStatusSerializer._fromJsonObject(object["status"]),
      statusText: MessageStatusTextSerializer._fromJsonObject(object["statusText"]),
      uid: object["uid"],
      updatedAt: new Date(object["updatedAt"]),
      url: object["url"],
      version: object["version"],
    };
  },

  _toJsonObject(self: MessageEndpointOut): any {
    return {
      channels: self.channels,
      createdAt: self.createdAt,
      description: self.description,
      disabled: self.disabled,
      filterTypes: self.filterTypes,
      id: self.id,
      nextAttempt: self.nextAttempt,
      rateLimit: self.rateLimit,
      status: MessageStatusSerializer._toJsonObject(self.status),
      statusText: MessageStatusTextSerializer._toJsonObject(self.statusText),
      uid: self.uid,
      updatedAt: self.updatedAt,
      url: self.url,
      version: self.version,
    };
  },
};
