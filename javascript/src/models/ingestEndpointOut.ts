// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface IngestEndpointOut {
  createdAt: Date;
  /** An example endpoint name. */
  description: string;
  disabled?: boolean;
  /** The Endpoint's ID. */
  id: string;
  metadata: { [key: string]: string };
  rateLimit?: number | null;
  /** Optional unique identifier for the endpoint. */
  uid?: string | null;
  updatedAt: Date;
  url: string;
}

export const IngestEndpointOutSerializer = {
  _fromJsonObject(object: any): IngestEndpointOut {
    return {
      createdAt: new Date(object["createdAt"]),
      description: object["description"],
      disabled: object["disabled"],
      id: object["id"],
      metadata: object["metadata"],
      rateLimit: object["rateLimit"],
      uid: object["uid"],
      updatedAt: new Date(object["updatedAt"]),
      url: object["url"],
    };
  },

  _toJsonObject(self: IngestEndpointOut): any {
    return {
      createdAt: self.createdAt,
      description: self.description,
      disabled: self.disabled,
      id: self.id,
      metadata: self.metadata,
      rateLimit: self.rateLimit,
      uid: self.uid,
      updatedAt: self.updatedAt,
      url: self.url,
    };
  },
};
