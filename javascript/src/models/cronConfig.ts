// this file is @generated

export interface CronConfig {
  schedule: string;
  payload: string;
  /**
   * Override the default content-type.
   *
   * Recommended if the payload is not JSON.
   */
  contentType?: string | null;
}

export const CronConfigSerializer = {
  _fromJsonObject(object: any): CronConfig {
    return {
      schedule: object["schedule"],
      payload: object["payload"],
      contentType: object["contentType"],
    };
  },

  _toJsonObject(self: CronConfig): any {
    return {
      schedule: self.schedule,
      payload: self.payload,
      contentType: self.contentType,
    };
  },
};
