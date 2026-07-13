// this file is @generated
import { type ConnectorKind, ConnectorKindSerializer } from "./connectorKind";
import { type ConnectorProduct, ConnectorProductSerializer } from "./connectorProduct";

export interface ConnectorIn {
  name: string;
  /** The Connector's UID. */
  uid?: string | null;
  logo?: string | null;
  description?: string;
  kind?: ConnectorKind;
  instructions?: string;
  allowedEventTypes?: string[] | null;
  transformation: string;
  featureFlags?: string[] | null;
  productType?: ConnectorProduct | null;
}

export const ConnectorInSerializer = {
  _fromJsonObject(object: any): ConnectorIn {
    return {
      name: object["name"],
      uid: object["uid"],
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
      productType:
        object["productType"] != null
          ? ConnectorProductSerializer._fromJsonObject(object["productType"])
          : undefined,
    };
  },

  _toJsonObject(self: ConnectorIn): any {
    return {
      name: self.name,
      uid: self.uid,
      logo: self.logo,
      description: self.description,
      kind:
        self.kind != null ? ConnectorKindSerializer._toJsonObject(self.kind) : undefined,
      instructions: self.instructions,
      allowedEventTypes: self.allowedEventTypes,
      transformation: self.transformation,
      featureFlags: self.featureFlags,
      productType:
        self.productType != null
          ? ConnectorProductSerializer._toJsonObject(self.productType)
          : undefined,
    };
  },
};
