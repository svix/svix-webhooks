// this file is @generated
import { type ApplicationIn, ApplicationInSerializer } from "./applicationIn";

export interface MessageIn {
  /** Optional unique identifier for the message */
  eventId?: string | null;
  /** The event type's name */
  eventType: string;
  /**
   * JSON payload to send as the request body of the webhook.
   *
   * We also support sending non-JSON payloads. Please contact us for more information.
   */
  payload: any;
  /** List of free-form identifiers that endpoints can filter by */
  channels?: string[] | null;
  /**
   * Optionally creates a new application alongside the message.
   *
   * If the application id or uid that is used in the path already exists, this argument is ignored.
   */
  application?: ApplicationIn | null;
  /** List of free-form tags that can be filtered by when listing messages */
  tags?: string[] | null;
  /** Extra parameters to pass to Transformations (for future use) */
  transformationsParams?: any | null;
  /**
   * The date and time at which the message will be delivered.
   *
   * Note that this time is best-effort-only. Must be at least one minute and no more than 24 hours in the future.
   */
  deliverAt?: Date | null;
  /** Optional number of days to retain the message payload. Defaults to 90. Note that this is mutually exclusive with `payloadRetentionHours`. */
  payloadRetentionPeriod?: number | null;
  /** Optional number of hours to retain the message payload. Note that this is mutually exclusive with `payloadRetentionPeriod`. */
  payloadRetentionHours?: number | null;
}

export const MessageInSerializer = {
  _fromJsonObject(object: any): MessageIn {
    return {
      eventId: object["eventId"],
      eventType: object["eventType"],
      payload: object["payload"],
      channels: object["channels"],
      application:
        object["application"] != null
          ? ApplicationInSerializer._fromJsonObject(object["application"])
          : undefined,
      tags: object["tags"],
      transformationsParams: object["transformationsParams"],
      deliverAt: object["deliverAt"] ? new Date(object["deliverAt"]) : null,
      payloadRetentionPeriod: object["payloadRetentionPeriod"],
      payloadRetentionHours: object["payloadRetentionHours"],
    };
  },

  _toJsonObject(self: MessageIn): any {
    return {
      eventId: self.eventId,
      eventType: self.eventType,
      payload: self.payload,
      channels: self.channels,
      application:
        self.application != null
          ? ApplicationInSerializer._toJsonObject(self.application)
          : undefined,
      tags: self.tags,
      transformationsParams: self.transformationsParams,
      deliverAt: self.deliverAt,
      payloadRetentionPeriod: self.payloadRetentionPeriod,
      payloadRetentionHours: self.payloadRetentionHours,
    };
  },
};
