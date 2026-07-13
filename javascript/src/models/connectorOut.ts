// this file is @generated
import { type ConnectorKind, ConnectorKindSerializer } from "./connectorKind";
import { type ConnectorProduct, ConnectorProductSerializer } from "./connectorProduct";

export interface ConnectorOut {
  /** The Connector's ID. */
  id: string;
  /** The Environment's ID. */
  orgId: string;
  /** The Connector's UID. */
  uid?: string | null;
  kind: ConnectorKind;
  name: string;
  logo?: string | null;
  description: string;
  instructions: string;
  allowedEventTypes?: string[] | null;
  transformation: string;
  createdAt: Date;
  updatedAt: Date;
  transformationUpdatedAt: Date;
  featureFlags?: string[] | null;
  productType: ConnectorProduct;
}

export const ConnectorOutSerializer = {
  _fromJsonObject(object: any): ConnectorOut {
    return {
      id: object["id"],
      orgId: object["orgId"],
      uid: object["uid"],
      kind: ConnectorKindSerializer._fromJsonObject(object["kind"]),
      name: object["name"],
      logo: object["logo"],
      description: object["description"],
      instructions: object["instructions"],
      allowedEventTypes: object["allowedEventTypes"],
      transformation: object["transformation"],
      createdAt: new Date(object["createdAt"]),
      updatedAt: new Date(object["updatedAt"]),
      transformationUpdatedAt: new Date(object["transformationUpdatedAt"]),
      featureFlags: object["featureFlags"],
      productType: ConnectorProductSerializer._fromJsonObject(object["productType"]),
    };
  },

  _toJsonObject(self: ConnectorOut): any {
    return {
      id: self.id,
      orgId: self.orgId,
      uid: self.uid,
      kind: ConnectorKindSerializer._toJsonObject(self.kind),
      name: self.name,
      logo: self.logo,
      description: self.description,
      instructions: self.instructions,
      allowedEventTypes: self.allowedEventTypes,
      transformation: self.transformation,
      createdAt: self.createdAt,
      updatedAt: self.updatedAt,
      transformationUpdatedAt: self.transformationUpdatedAt,
      featureFlags: self.featureFlags,
      productType: ConnectorProductSerializer._toJsonObject(self.productType),
    };
  },
};
