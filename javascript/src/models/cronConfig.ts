// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface CronConfig {
  /**
   * Override the default content-type.
   *
   * Recommended if the payload is not JSON.
   */
  contentType?: string | null;
  payload: string;
  schedule: string;
}

export const CronConfigSerializer = {
  _fromJsonObject(object: any): CronConfig {
    return {
      contentType: object["contentType"],
      payload: object["payload"],
      schedule: object["schedule"],
    };
  },

  _toJsonObject(self: CronConfig): any {
    return {
      contentType: self.contentType,
      payload: self.payload,
      schedule: self.schedule,
    };
  },
};
