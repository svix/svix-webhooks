// this file is @generated

/** Configuration parameters for defining a Segment sink. */
export interface SegmentSinkConfigIn {
  /**
   * The Segment ingestion host.
   *
   * This is https://api.segment.io for US workspaces, https://events.eu1.segmentapis.com
   * for EU workspaces, or your custom domain. Events are sent to the `/v1/batch` endpoint
   * under this host.
   */
  host: string;
  /**
   * The Segment source write key.
   *
   * Sent as HTTP Basic auth (the write key as the username, with an empty password) on each
   * `/v1/batch` request.
   */
  writeKey: string;
}

export const SegmentSinkConfigInSerializer = {
  _fromJsonObject(object: any): SegmentSinkConfigIn {
    return {
      host: object["host"],
      writeKey: object["writeKey"],
    };
  },

  _toJsonObject(self: SegmentSinkConfigIn): any {
    return {
      host: self.host,
      writeKey: self.writeKey,
    };
  },
};
