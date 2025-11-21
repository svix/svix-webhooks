// this file is @generated
import { type ConnectorKind, ConnectorKindSerializer } from "./connectorKind";
import { type ConnectorProduct, ConnectorProductSerializer } from "./connectorProduct";

export interface ConnectorIn {
  allowedEventTypes?: string[] | null;
  description?: string;
  featureFlags?: string[] | null;
  instructions?: string;
  kind?: ConnectorKind;
  logo?: string | null;
  name: string;
  productType?: ConnectorProduct | null;
  transformation: string;
  /** The Connector's UID. */
  uid?: string | null;
}

export const ConnectorInSerializer = {
  _fromJsonObject(object: any): ConnectorIn {
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
      productType: object["productType"]
        ? ConnectorProductSerializer._fromJsonObject(object["productType"])
        : undefined,
      transformation: object["transformation"],
      uid: object["uid"],
    };
  },

  _toJsonObject(self: ConnectorIn): any {
    return {
      allowedEventTypes: self.allowedEventTypes,
      description: self.description,
      featureFlags: self.featureFlags,
      instructions: self.instructions,
      kind: self.kind ? ConnectorKindSerializer._toJsonObject(self.kind) : undefined,
      logo: self.logo,
      name: self.name,
      productType: self.productType
        ? ConnectorProductSerializer._toJsonObject(self.productType)
        : undefined,
      transformation: self.transformation,
      uid: self.uid,
    };
  },
};
