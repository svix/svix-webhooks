// this file is @generated
import { type ConnectorKind, ConnectorKindSerializer } from "./connectorKind";

export interface ConnectorUpdate {
  allowedEventTypes?: string[] | null;
  description?: string;
  featureFlags?: string[] | null;
  instructions?: string;
  kind?: ConnectorKind;
  logo?: string | null;
  name?: string;
  transformation: string;
}

export const ConnectorUpdateSerializer = {
  _fromJsonObject(object: any): ConnectorUpdate {
    return {
      allowedEventTypes: object["allowedEventTypes"],
      description: object["description"],
      featureFlags: object["featureFlags"],
      instructions: object["instructions"],
      kind: object["kind"]
        ? ConnectorKindSerializer._fromJsonObject(object["kind"])
        : undefined,
      logo: object["logo"],
      name: object["name"],
      transformation: object["transformation"],
    };
  },

  _toJsonObject(self: ConnectorUpdate): any {
    return {
      allowedEventTypes: self.allowedEventTypes,
      description: self.description,
      featureFlags: self.featureFlags,
      instructions: self.instructions,
      kind: self.kind ? ConnectorKindSerializer._toJsonObject(self.kind) : undefined,
      logo: self.logo,
      name: self.name,
      transformation: self.transformation,
    };
  },
};
