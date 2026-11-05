// this file is @generated
import {
  type GenericWebhookResponse,
  GenericWebhookResponseSerializer,
} from "./genericWebhookResponse";

export interface GenericWebhookConfigOut {
  response?: GenericWebhookResponse | null;
}

export const GenericWebhookConfigOutSerializer = {
  _fromJsonObject(object: any): GenericWebhookConfigOut {
    return {
      response:
        object["response"] != null
          ? GenericWebhookResponseSerializer._fromJsonObject(object["response"])
          : undefined,
    };
  },

  _toJsonObject(self: GenericWebhookConfigOut): any {
    return {
      response:
        self.response != null
          ? GenericWebhookResponseSerializer._toJsonObject(self.response)
          : undefined,
    };
  },
};
