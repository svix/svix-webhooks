// this file is @generated

export interface EventBridgeConfigIn {
  /** The name or ARN of the event bus to receive the event */
  eventBusName: string;
  /** Free-form string, with a maximum of 128 characters */
  detailType?: string;
  /**
   * Access key ID.
   *
   * Required (along with `secret_access_key`) if `role_arn` is blank.
   */
  accessKeyId?: string | null;
  /**
   * Secret access key.
   *
   * Required (along with `access_key_id`) if `role_arn` is blank.
   */
  secretAccessKey?: string | null;
  /** Role ARN for delegated authentication */
  roleArn?: string | null;
  /**
   * Shared secret passed as the STS ExternalId.
   *
   * Can only be set if `role_arn` is Some
   */
  externalId?: string | null;
  /**
   * The region of the EventBridge bus.
   *
   * Currently a required field, but marked as optional because we may infer it from other fields in the future.
   */
  region?: string | null;
}

export const EventBridgeConfigInSerializer = {
  _fromJsonObject(object: any): EventBridgeConfigIn {
    return {
      eventBusName: object["eventBusName"],
      detailType: object["detailType"],
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      roleArn: object["roleArn"],
      externalId: object["externalId"],
      region: object["region"],
    };
  },

  _toJsonObject(self: EventBridgeConfigIn): any {
    return {
      eventBusName: self.eventBusName,
      detailType: self.detailType,
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      roleArn: self.roleArn,
      externalId: self.externalId,
      region: self.region,
    };
  },
};
