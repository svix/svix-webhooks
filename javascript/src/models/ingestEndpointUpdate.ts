// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface IngestEndpointUpdate {
  description?: string;
  disabled?: boolean;
  metadata?: { [key: string]: string };
  rateLimit?: number | null;
  /** Optional unique identifier for the endpoint. */
  uid?: string | null;
  url: string;
}

export const IngestEndpointUpdateSerializer = {
  _fromJsonObject(object: any): IngestEndpointUpdate {
    return {
      description: object["description"],
      disabled: object["disabled"],
      metadata: object["metadata"],
      rateLimit: object["rateLimit"],
      uid: object["uid"],
      url: object["url"],
    };
  },

  _toJsonObject(self: IngestEndpointUpdate): any {
    return {
      description: self.description,
      disabled: self.disabled,
      metadata: self.metadata,
      rateLimit: self.rateLimit,
      uid: self.uid,
      url: self.url,
    };
  },
};
