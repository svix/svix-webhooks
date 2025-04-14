// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { EnvironmentType, EnvironmentTypeSerializer } from "./environmentType";

export interface EnvironmentModelIn {
  name: string;
  type: EnvironmentType;
}

export const EnvironmentModelInSerializer = {
  _fromJsonObject(object: any): EnvironmentModelIn {
    return {
      name: object["name"],
      type: EnvironmentTypeSerializer._fromJsonObject(object["type"]),
    };
  },

  _toJsonObject(self: EnvironmentModelIn): any {
    return {
      name: self.name,
      type: EnvironmentTypeSerializer._toJsonObject(self.type),
    };
  },
};
