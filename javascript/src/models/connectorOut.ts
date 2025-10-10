// this file is @generated
import { type ConnectorKind, ConnectorKindSerializer } from "./connectorKind";

export interface ConnectorOut {
  createdAt: Date;
  description: string;
  featureFlags?: string[] | null;
  filterTypes?: string[] | null;
  /** The Connector's ID. */
  id: string;
  instructions: string;
  kind: ConnectorKind;
  logo: string;
  name: string;
  /** The Environment's ID. */
  orgId: string;
  transformation: string;
  updatedAt: Date;
}

export const ConnectorOutSerializer = {
  _fromJsonObject(object: any): ConnectorOut {
    return {
      createdAt: new Date(object["createdAt"]),
      description: object["description"],
      featureFlags: object["featureFlags"],
      filterTypes: object["filterTypes"],
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
      createdAt: self.createdAt,
      description: self.description,
      featureFlags: self.featureFlags,
      filterTypes: self.filterTypes,
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
