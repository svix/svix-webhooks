// this file is @generated
import {
  type McpEventsDeliveryMode,
  McpEventsDeliveryModeSerializer,
} from "./mcpEventsDeliveryMode";

export interface McpEventsUnsubscribeDeliveryIn {
  /** How events are delivered. Only `webhook` is supported. */
  mode: McpEventsDeliveryMode;
  /** The callback URL the subscription was made with. */
  url: string;
}

export const McpEventsUnsubscribeDeliveryInSerializer = {
  _fromJsonObject(object: any): McpEventsUnsubscribeDeliveryIn {
    return {
      mode: McpEventsDeliveryModeSerializer._fromJsonObject(object["mode"]),
      url: object["url"],
    };
  },

  _toJsonObject(self: McpEventsUnsubscribeDeliveryIn): any {
    return {
      mode: McpEventsDeliveryModeSerializer._toJsonObject(self.mode),
      url: self.url,
    };
  },
};
