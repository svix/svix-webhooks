// this file is @generated
import { type ConnectorKind, ConnectorKindSerializer } from "./connectorKind";

export interface ConnectorUpsertIn {
  name?: string;
  logo?: string | null;
  description?: string;
  kind?: ConnectorKind;
  instructions?: string;
  allowedEventTypes?: string[] | null;
  transformation: string;
  featureFlags?: string[] | null;
}

export const ConnectorUpsertInSerializer = {
  _fromJsonObject(object: any): ConnectorUpsertIn {
    return {
      name: object["name"],
      logo: object["logo"],
      description: object["description"],
      kind:
        object["kind"] != null
          ? ConnectorKindSerializer._fromJsonObject(object["kind"])
          : undefined,
      instructions: object["instructions"],
      allowedEventTypes: object["allowedEventTypes"],
      transformation: object["transformation"],
      featureFlags: object["featureFlags"],
    };
  },

  _toJsonObject(self: ConnectorUpsertIn): any {
    return {
      name: self.name,
      logo: self.logo,
      description: self.description,
      kind:
        self.kind != null ? ConnectorKindSerializer._toJsonObject(self.kind) : undefined,
      instructions: self.instructions,
      allowedEventTypes: self.allowedEventTypes,
      transformation: self.transformation,
      featureFlags: self.featureFlags,
    };
  },
};
