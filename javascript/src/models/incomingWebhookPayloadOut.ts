// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface IncomingWebhookPayloadOut {
  channel?: string | null;
  error?: string | null;
  incomingWebhookUrl?: string | null;
}

export const IncomingWebhookPayloadOutSerializer = {
  _fromJsonObject(object: any): IncomingWebhookPayloadOut {
    return {
      channel: object["channel"],
      error: object["error"],
      incomingWebhookUrl: object["incomingWebhookUrl"],
    };
  },

  _toJsonObject(self: IncomingWebhookPayloadOut): any {
    return {
      channel: self.channel,
      error: self.error,
      incomingWebhookUrl: self.incomingWebhookUrl,
    };
  },
};
