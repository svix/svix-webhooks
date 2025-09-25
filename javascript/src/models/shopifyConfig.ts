// this file is @generated

export interface ShopifyConfig {
  secret: string;
}

export const ShopifyConfigSerializer = {
  _fromJsonObject(object: any): ShopifyConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: ShopifyConfig): any {
    return {
      secret: self.secret,
    };
  },
};
