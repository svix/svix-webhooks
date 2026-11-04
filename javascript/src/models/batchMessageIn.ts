// this file is @generated

/**
 * A single message in a batch of bulk-created messages
 *
 * Practically identical to `MessageIn`, but without the `application` field
 */
export interface BatchMessageIn {
  /**
   * Optional unique identifier for the message
   *
   * This should not contain the '.' character
   */
  eventId?: string | null;
  /** The event type's name */
  eventType: string;
  /**
   * JSON payload to send as the request body of the webhook.
   *
   * We also support sending non-JSON payloads. Please contact us for more information.
   *
   * The serialized byte-length of this value should be under 1MiB. Please contact us
   * if you need to send payloads large than 1MiB.
   */
  payload: any;
  /** List of free-form identifiers that endpoints can filter by */
  channels?: string[] | null;
  /** List of free-form tags that can be filtered by when listing messages */
  tags?: string[] | null;
  /** Extra parameters to pass to Transformations (for future use) */
  transformationsParams?: any | null;
  /**
   * The date and time at which the message will be delivered.
   *
   * Note that this time is best-effort-only. Must be at least one minute
   * and no more than 24 hours in the future.
   */
  deliverAt?: Date | null;
  /**
   * Optional number of days to retain the message payload. Defaults to 90.
   * Note that this is mutually exclusive with `payloadRetentionHours`.
   */
  payloadRetentionPeriod?: number | null;
  /**
   * Optional number of hours to retain the message payload. Note that
   * this is mutually exclusive with `payloadRetentionPeriod`.
   */
  payloadRetentionHours?: number | null;
}

export const BatchMessageInSerializer = {
  _fromJsonObject(object: any): BatchMessageIn {
    return {
      eventId: object["eventId"],
      eventType: object["eventType"],
      payload: object["payload"],
      channels: object["channels"],
      tags: object["tags"],
      transformationsParams: object["transformationsParams"],
      deliverAt: object["deliverAt"] ? new Date(object["deliverAt"]) : null,
      payloadRetentionPeriod: object["payloadRetentionPeriod"],
      payloadRetentionHours: object["payloadRetentionHours"],
    };
  },

  _toJsonObject(self: BatchMessageIn): any {
    return {
      eventId: self.eventId,
      eventType: self.eventType,
      payload: self.payload,
      channels: self.channels,
      tags: self.tags,
      transformationsParams: self.transformationsParams,
      deliverAt: self.deliverAt,
      payloadRetentionPeriod: self.payloadRetentionPeriod,
      payloadRetentionHours: self.payloadRetentionHours,
    };
  },
};
