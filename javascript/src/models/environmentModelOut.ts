// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { EnvironmentRegion, EnvironmentRegionSerializer } from "./environmentRegion";
import { EnvironmentType, EnvironmentTypeSerializer } from "./environmentType";

export interface EnvironmentModelOut {
  createdAt: Date;
  /** The Environment's ID. */
  id: string;
  name: string;
  region: EnvironmentRegion;
  type: EnvironmentType;
  updatedAt: Date;
}

export const EnvironmentModelOutSerializer = {
  _fromJsonObject(object: any): EnvironmentModelOut {
    return {
      createdAt: new Date(object["createdAt"]),
      id: object["id"],
      name: object["name"],
      region: EnvironmentRegionSerializer._fromJsonObject(object["region"]),
      type: EnvironmentTypeSerializer._fromJsonObject(object["type"]),
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: EnvironmentModelOut): any {
    return {
      createdAt: self.createdAt,
      id: self.id,
      name: self.name,
      region: EnvironmentRegionSerializer._toJsonObject(self.region),
      type: EnvironmentTypeSerializer._toJsonObject(self.type),
      updatedAt: self.updatedAt,
    };
  },
};
