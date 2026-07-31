// this file is @generated
import {
  type MessageAttemptTriggerType,
  MessageAttemptTriggerTypeSerializer,
} from "./messageAttemptTriggerType";
import { type MessageOut, MessageOutSerializer } from "./messageOut";
import { type MessageStatus, MessageStatusSerializer } from "./messageStatus";
import { type MessageStatusText, MessageStatusTextSerializer } from "./messageStatusText";

export interface MessageAttemptOut {
  url: string;
  response: string;
  responseStatusCode: number;
  /** Response duration in milliseconds. */
  responseDurationMs: number;
  status: MessageStatus;
  statusText: MessageStatusText;
  triggerType: MessageAttemptTriggerType;
  /** The Message's ID. */
  msgId: string;
  /** The Endpoint's ID. */
  endpointId: string;
  /** The MessageAttempt's ID. */
  id: string;
  timestamp: Date;
  msg?: MessageOut | null;
}

export const MessageAttemptOutSerializer = {
  _fromJsonObject(object: any): MessageAttemptOut {
    return {
      url: object["url"],
      response: object["response"],
      responseStatusCode: object["responseStatusCode"],
      responseDurationMs: object["responseDurationMs"],
      status: MessageStatusSerializer._fromJsonObject(object["status"]),
      statusText: MessageStatusTextSerializer._fromJsonObject(object["statusText"]),
      triggerType: MessageAttemptTriggerTypeSerializer._fromJsonObject(
        object["triggerType"]
      ),
      msgId: object["msgId"],
      endpointId: object["endpointId"],
      id: object["id"],
      timestamp: new Date(object["timestamp"]),
      msg:
        object["msg"] != null
          ? MessageOutSerializer._fromJsonObject(object["msg"])
          : undefined,
    };
  },

  _toJsonObject(self: MessageAttemptOut): any {
    return {
      url: self.url,
      response: self.response,
      responseStatusCode: self.responseStatusCode,
      responseDurationMs: self.responseDurationMs,
      status: MessageStatusSerializer._toJsonObject(self.status),
      statusText: MessageStatusTextSerializer._toJsonObject(self.statusText),
      triggerType: MessageAttemptTriggerTypeSerializer._toJsonObject(self.triggerType),
      msgId: self.msgId,
      endpointId: self.endpointId,
      id: self.id,
      timestamp: self.timestamp,
      msg: self.msg != null ? MessageOutSerializer._toJsonObject(self.msg) : undefined,
    };
  },
};
