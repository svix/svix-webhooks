// this file is @generated
import { type ConnectorKind, ConnectorKindSerializer } from "./connectorKind";

export interface ConnectorIn {
  description?: string;
  featureFlags?: string[] | null;
  filterTypes?: string[] | null;
  instructions?: string;
  kind?: ConnectorKind;
  logo: string;
  name: string;
  transformation: string;
}

export const ConnectorInSerializer = {
  _fromJsonObject(object: any): ConnectorIn {
    return {
      description: object["description"],
      featureFlags: object["featureFlags"],
      filterTypes: object["filterTypes"],
      instructions: object["instructions"],
      kind: object["kind"]
        ? ConnectorKindSerializer._fromJsonObject(object["kind"])
        : undefined,
      logo: object["logo"],
      name: object["name"],
      transformation: object["transformation"],
    };
  },

  _toJsonObject(self: ConnectorIn): any {
    return {
      description: self.description,
      featureFlags: self.featureFlags,
      filterTypes: self.filterTypes,
      instructions: self.instructions,
      kind: self.kind ? ConnectorKindSerializer._toJsonObject(self.kind) : undefined,
      logo: self.logo,
      name: self.name,
      transformation: self.transformation,
    };
  },
};
