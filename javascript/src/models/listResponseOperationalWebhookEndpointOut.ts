// this file is @generated
import {
  type OperationalWebhookEndpointOut,
  OperationalWebhookEndpointOutSerializer,
} from "./operationalWebhookEndpointOut";

export interface ListResponseOperationalWebhookEndpointOut {
  data: OperationalWebhookEndpointOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseOperationalWebhookEndpointOutSerializer = {
  _fromJsonObject(object: any): ListResponseOperationalWebhookEndpointOut {
    return {
      data: object["data"].map((item: OperationalWebhookEndpointOut) =>
        OperationalWebhookEndpointOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseOperationalWebhookEndpointOut): any {
    return {
      data: self.data.map((item) =>
        OperationalWebhookEndpointOutSerializer._toJsonObject(item)
      ),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
