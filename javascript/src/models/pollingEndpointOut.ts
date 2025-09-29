// this file is @generated
import {
  type PollingEndpointMessageOut,
  PollingEndpointMessageOutSerializer,
} from "./pollingEndpointMessageOut";

export interface PollingEndpointOut {
  data: PollingEndpointMessageOut[];
  done: boolean;
  iterator: string;
}

export const PollingEndpointOutSerializer = {
  _fromJsonObject(object: any): PollingEndpointOut {
    return {
      data: object["data"].map((item: PollingEndpointMessageOut) =>
        PollingEndpointMessageOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
    };
  },

  _toJsonObject(self: PollingEndpointOut): any {
    return {
      data: self.data.map((item) =>
        PollingEndpointMessageOutSerializer._toJsonObject(item)
      ),
      done: self.done,
      iterator: self.iterator,
    };
  },
};
