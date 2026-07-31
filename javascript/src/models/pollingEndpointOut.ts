// this file is @generated
import {
  type PollingEndpointMessageOut,
  PollingEndpointMessageOutSerializer,
} from "./pollingEndpointMessageOut";

export interface PollingEndpointOut {
  data: PollingEndpointMessageOut[];
  iterator: string;
  done: boolean;
}

export const PollingEndpointOutSerializer = {
  _fromJsonObject(object: any): PollingEndpointOut {
    return {
      data: object["data"].map((item: PollingEndpointMessageOut) =>
        PollingEndpointMessageOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: PollingEndpointOut): any {
    return {
      data: self.data.map((item) =>
        PollingEndpointMessageOutSerializer._toJsonObject(item)
      ),
      iterator: self.iterator,
      done: self.done,
    };
  },
};
