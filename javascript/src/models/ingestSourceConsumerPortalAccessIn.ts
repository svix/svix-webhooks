// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface IngestSourceConsumerPortalAccessIn {
  /**
   * How long the token will be valid for, in seconds.
   *
   * Valid values are between 1 hour and 7 days. The default is 7 days.
   */
  expiry?: number | null;
  /** Whether the app portal should be in read-only mode. */
  readOnly?: boolean | null;
}

export const IngestSourceConsumerPortalAccessInSerializer = {
  _fromJsonObject(object: any): IngestSourceConsumerPortalAccessIn {
    return {
      expiry: object["expiry"],
      readOnly: object["readOnly"],
    };
  },

  _toJsonObject(self: IngestSourceConsumerPortalAccessIn): any {
    return {
      expiry: self.expiry,
      readOnly: self.readOnly,
    };
  },
};
