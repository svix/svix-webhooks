// this file is @generated
import {
  type McpEventsUnsubscribeDeliveryIn,
  McpEventsUnsubscribeDeliveryInSerializer,
} from "./mcpEventsUnsubscribeDeliveryIn";

export interface McpEventsUnsubscribeIn {
  /** The event the subscription was made for. */
  name: string;
  /**
   * The filter arguments the subscription was made with, as given to
   * `events/subscribe`. See [Stop a subscription](https://developers.openai.com/plugins/build/mcp-events#stop-a-subscription)
   * in OpenAI's MCP Events docs.
   */
  arguments?: any;
  delivery: McpEventsUnsubscribeDeliveryIn;
}

export const McpEventsUnsubscribeInSerializer = {
  _fromJsonObject(object: any): McpEventsUnsubscribeIn {
    return {
      name: object["name"],
      arguments: object["arguments"],
      delivery: McpEventsUnsubscribeDeliveryInSerializer._fromJsonObject(
        object["delivery"]
      ),
    };
  },

  _toJsonObject(self: McpEventsUnsubscribeIn): any {
    return {
      name: self.name,
      arguments: self.arguments,
      delivery: McpEventsUnsubscribeDeliveryInSerializer._toJsonObject(self.delivery),
    };
  },
};
