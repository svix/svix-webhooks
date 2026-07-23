// this file is @generated

export interface EndpointPatch {
  channels?: string[] | null;
  description?: string;
  disabled?: boolean;
  filterTypes?: string[] | null;
  metadata?: { [key: string]: string };
  /**
   * Maximum messages per second to send to this endpoint.
   *
   * Outgoing messages will be throttled to this rate.
   */
  throttleRate?: number | null;
  /** The Endpoint's UID. */
  uid?: string | null;
  url?: string;
}

export const EndpointPatchSerializer = {
  _fromJsonObject(object: any): EndpointPatch {
    return {
      channels: object["channels"],
      description: object["description"],
      disabled: object["disabled"],
      filterTypes: object["filterTypes"],
      metadata: object["metadata"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      url: object["url"],
    };
  },

  _toJsonObject(self: EndpointPatch): any {
    return {
      channels: self.channels,
      description: self.description,
      disabled: self.disabled,
      filterTypes: self.filterTypes,
      metadata: self.metadata,
      throttleRate: self.throttleRate,
      uid: self.uid,
      url: self.url,
    };
  },
};
