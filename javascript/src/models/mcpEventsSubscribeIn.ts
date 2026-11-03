// this file is @generated
import {
  type EndpointChallengeMode,
  EndpointChallengeModeSerializer,
} from "./endpointChallengeMode";
import {
  type McpEventsSubscribeDeliveryIn,
  McpEventsSubscribeDeliveryInSerializer,
} from "./mcpEventsSubscribeDeliveryIn";

export interface McpEventsSubscribeIn {
  /** The event to subscribe to. It must be an existing event type. */
  name: string;
  /**
   * The filter arguments of the `events/subscribe` request, as described by
   * the event's `inputSchema` (e.g. `{"document_id": "doc_123"}`).
   * Together with `name` and the callback URL they identify the subscription
   *
   * They're stored in the endpoint's `mcpEventArguments` metadata, but
   * aren't used to filter messages; use `channels` for that.
   *
   * See [Create a subscription](https://developers.openai.com/plugins/build/mcp-events#create-a-subscription)
   * in OpenAI's MCP Events docs.
   */
  arguments?: any;
  delivery: McpEventsSubscribeDeliveryIn;
  /**
   * The requested lifetime of the subscription, in milliseconds. Omit for
   * the default of 7 days, or set to `null` for a subscription that never
   * expires.
   */
  ttlMs?: number | null;
  /**
   * How long the previous secret keeps signing when a refresh changes it,
   * in seconds.
   *
   * Valid values are between 0 (immediate expiry) and 7 days. The default
   * is 5 minutes.
   */
  secretGracePeriodSeconds?: number | null;
  /** How the callback must respond to the verification challenge. */
  challengeMode?: EndpointChallengeMode;
  /**
   * Only send messages with one of these channels to the subscription,
   * for example ones derived from `arguments`. Omit to send all messages
   * of the event type.
   */
  channels?: string[] | null;
}

export const McpEventsSubscribeInSerializer = {
  _fromJsonObject(object: any): McpEventsSubscribeIn {
    return {
      name: object["name"],
      arguments: object["arguments"],
      delivery: McpEventsSubscribeDeliveryInSerializer._fromJsonObject(
        object["delivery"]
      ),
      ttlMs: object["ttlMs"],
      secretGracePeriodSeconds: object["secretGracePeriodSeconds"],
      challengeMode:
        object["challengeMode"] != null
          ? EndpointChallengeModeSerializer._fromJsonObject(object["challengeMode"])
          : undefined,
      channels: object["channels"],
    };
  },

  _toJsonObject(self: McpEventsSubscribeIn): any {
    return {
      name: self.name,
      arguments: self.arguments,
      delivery: McpEventsSubscribeDeliveryInSerializer._toJsonObject(self.delivery),
      ttlMs: self.ttlMs,
      secretGracePeriodSeconds: self.secretGracePeriodSeconds,
      challengeMode:
        self.challengeMode != null
          ? EndpointChallengeModeSerializer._toJsonObject(self.challengeMode)
          : undefined,
      channels: self.channels,
    };
  },
};
