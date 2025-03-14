// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  TransformationHttpMethod,
  TransformationHttpMethodSerializer,
} from "./transformationHttpMethod";

export interface TransformationSimulateOut {
  method?: TransformationHttpMethod | null;
  payload: string;
  url: string;
}

export const TransformationSimulateOutSerializer = {
  _fromJsonObject(object: any): TransformationSimulateOut {
    return {
      method: object["method"]
        ? TransformationHttpMethodSerializer._fromJsonObject(object["method"])
        : undefined,
      payload: object["payload"],
      url: object["url"],
    };
  },

  _toJsonObject(self: TransformationSimulateOut): any {
    return {
      method: self.method
        ? TransformationHttpMethodSerializer._toJsonObject(self.method)
        : undefined,
      payload: self.payload,
      url: self.url,
    };
  },
};
