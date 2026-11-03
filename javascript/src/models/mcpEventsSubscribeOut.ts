// this file is @generated

export interface McpEventsSubscribeOut {
  /** The subscription's ID, which is also the UID of its endpoint. */
  id: string;
  /** The Endpoint's ID. */
  endpointId: string;
  /** When the subscription expires, or `null` if it never does. */
  refreshBefore?: Date | null;
  /** Always `null`, since events can't be replayed. */
  cursor?: string | null;
  /** Always `false`, since events can't be replayed. */
  truncated: boolean;
}

export const McpEventsSubscribeOutSerializer = {
  _fromJsonObject(object: any): McpEventsSubscribeOut {
    return {
      id: object["id"],
      endpointId: object["endpointId"],
      refreshBefore: object["refreshBefore"] ? new Date(object["refreshBefore"]) : null,
      cursor: object["cursor"],
      truncated: object["truncated"],
    };
  },

  _toJsonObject(self: McpEventsSubscribeOut): any {
    return {
      id: self.id,
      endpointId: self.endpointId,
      refreshBefore: self.refreshBefore,
      cursor: self.cursor,
      truncated: self.truncated,
    };
  },
};
