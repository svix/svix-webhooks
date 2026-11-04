// this file is @generated
import {
  type McpEventsDeliveryMode,
  McpEventsDeliveryModeSerializer,
} from "./mcpEventsDeliveryMode";

export interface McpEventsSubscribeDeliveryIn {
  /** How events are delivered. Only `webhook` is supported. */
  mode: McpEventsDeliveryMode;
  /** The callback URL events are sent to. */
  url: string;
  /**
   * The secret events are signed with. MCP Events requires a `whsec_`
   * secret.
   */
  secret: string;
}

export const McpEventsSubscribeDeliveryInSerializer = {
  _fromJsonObject(object: any): McpEventsSubscribeDeliveryIn {
    return {
      mode: McpEventsDeliveryModeSerializer._fromJsonObject(object["mode"]),
      url: object["url"],
      secret: object["secret"],
    };
  },

  _toJsonObject(self: McpEventsSubscribeDeliveryIn): any {
    return {
      mode: McpEventsDeliveryModeSerializer._toJsonObject(self.mode),
      url: self.url,
      secret: self.secret,
    };
  },
};
