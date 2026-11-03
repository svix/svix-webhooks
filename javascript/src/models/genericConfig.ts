// this file is @generated
import { type GenericResponse, GenericResponseSerializer } from "./genericResponse";

export interface GenericConfig {
  response?: GenericResponse | null;
}

export const GenericConfigSerializer = {
  _fromJsonObject(object: any): GenericConfig {
    return {
      response:
        object["response"] != null
          ? GenericResponseSerializer._fromJsonObject(object["response"])
          : undefined,
    };
  },

  _toJsonObject(self: GenericConfig): any {
    return {
      response:
        self.response != null
          ? GenericResponseSerializer._toJsonObject(self.response)
          : undefined,
    };
  },
};
