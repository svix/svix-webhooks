// this file is @generated
import { type ConnectorKind, ConnectorKindSerializer } from "./connectorKind";

export interface ConnectorOut {
  allowedEventTypes?: string[] | null;
  createdAt: Date;
  description: string;
  featureFlags?: string[] | null;
  /** The Connector's ID. */
  id: string;
  instructions: string;
  kind: ConnectorKind;
  logo?: string | null;
  name: string;
  /** The Environment's ID. */
  orgId: string;
  transformation: string;
  updatedAt: Date;
}

export const ConnectorOutSerializer = {
  _fromJsonObject(object: any): ConnectorOut {
    return {
      allowedEventTypes: object["allowedEventTypes"],
      createdAt: new Date(object["createdAt"]),
      description: object["description"],
      featureFlags: object["featureFlags"],
      id: object["id"],
      instructions: object["instructions"],
      kind: ConnectorKindSerializer._fromJsonObject(object["kind"]),
      logo: object["logo"],
      name: object["name"],
      orgId: object["orgId"],
      transformation: object["transformation"],
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: ConnectorOut): any {
    return {
      allowedEventTypes: self.allowedEventTypes,
      createdAt: self.createdAt,
      description: self.description,
      featureFlags: self.featureFlags,
      id: self.id,
      instructions: self.instructions,
      kind: ConnectorKindSerializer._toJsonObject(self.kind),
      logo: self.logo,
      name: self.name,
      orgId: self.orgId,
      transformation: self.transformation,
      updatedAt: self.updatedAt,
    };
  },
};
