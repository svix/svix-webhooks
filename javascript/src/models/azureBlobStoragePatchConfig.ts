// this file is @generated

export interface AzureBlobStoragePatchConfig {
  accessKey?: string;
  account?: string;
  container?: string;
}

export const AzureBlobStoragePatchConfigSerializer = {
  _fromJsonObject(object: any): AzureBlobStoragePatchConfig {
    return {
      accessKey: object["accessKey"],
      account: object["account"],
      container: object["container"],
    };
  },

  _toJsonObject(self: AzureBlobStoragePatchConfig): any {
    return {
      accessKey: self.accessKey,
      account: self.account,
      container: self.container,
    };
  },
};
