// this file is @generated
import {
  type GenericWebhookResponse,
  GenericWebhookResponseSerializer,
} from "./genericWebhookResponse";

export interface GenericWebhookConfig {
  response?: GenericWebhookResponse | null;
}

export const GenericWebhookConfigSerializer = {
  _fromJsonObject(object: any): GenericWebhookConfig {
    return {
      response:
        object["response"] != null
          ? GenericWebhookResponseSerializer._fromJsonObject(object["response"])
          : undefined,
    };
  },

  _toJsonObject(self: GenericWebhookConfig): any {
    return {
      response:
        self.response != null
          ? GenericWebhookResponseSerializer._toJsonObject(self.response)
          : undefined,
    };
  },
};
