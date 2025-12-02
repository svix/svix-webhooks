// this file is @generated
import { type HttpAttemptTimes, HttpAttemptTimesSerializer } from "./httpAttemptTimes";
import { type MessageStatus, MessageStatusSerializer } from "./messageStatus";

export interface MessageAttemptLog {
  /** The Application's ID. */
  appId: string;
  /** The Application's UID. */
  appUid?: string | null;
  attemptCount: number;
  attemptEnd: Date;
  /** The MessageAttempt's ID. */
  attemptId: string;
  attemptStart: Date;
  /** The Endpoint's ID. */
  endpointId: string;
  /** The event type's name */
  eventType?: string | null;
  httpTimes?: HttpAttemptTimes | null;
  msgCreated: Date;
  /** The Message's UID. */
  msgEventId?: string | null;
  /** The Message's ID. */
  msgId: string;
  /** The Environment's ID. */
  orgId: string;
  responseStatusCode: number;
  status: MessageStatus;
}

export const MessageAttemptLogSerializer = {
  _fromJsonObject(object: any): MessageAttemptLog {
    return {
      appId: object["appId"],
      appUid: object["appUid"],
      attemptCount: object["attemptCount"],
      attemptEnd: new Date(object["attemptEnd"]),
      attemptId: object["attemptId"],
      attemptStart: new Date(object["attemptStart"]),
      endpointId: object["endpointId"],
      eventType: object["eventType"],
      httpTimes: object["httpTimes"]
        ? HttpAttemptTimesSerializer._fromJsonObject(object["httpTimes"])
        : undefined,
      msgCreated: new Date(object["msgCreated"]),
      msgEventId: object["msgEventId"],
      msgId: object["msgId"],
      orgId: object["orgId"],
      responseStatusCode: object["responseStatusCode"],
      status: MessageStatusSerializer._fromJsonObject(object["status"]),
    };
  },

  _toJsonObject(self: MessageAttemptLog): any {
    return {
      appId: self.appId,
      appUid: self.appUid,
      attemptCount: self.attemptCount,
      attemptEnd: self.attemptEnd,
      attemptId: self.attemptId,
      attemptStart: self.attemptStart,
      endpointId: self.endpointId,
      eventType: self.eventType,
      httpTimes: self.httpTimes
        ? HttpAttemptTimesSerializer._toJsonObject(self.httpTimes)
        : undefined,
      msgCreated: self.msgCreated,
      msgEventId: self.msgEventId,
      msgId: self.msgId,
      orgId: self.orgId,
      responseStatusCode: self.responseStatusCode,
      status: MessageStatusSerializer._toJsonObject(self.status),
    };
  },
};
