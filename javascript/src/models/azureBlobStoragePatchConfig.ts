// this file is @generated

export interface AzureBlobStoragePatchConfig {
  container?: string;
  account?: string;
  accessKey?: string;
}

export const AzureBlobStoragePatchConfigSerializer = {
  _fromJsonObject(object: any): AzureBlobStoragePatchConfig {
    return {
      container: object["container"],
      account: object["account"],
      accessKey: object["accessKey"],
    };
  },

  _toJsonObject(self: AzureBlobStoragePatchConfig): any {
    return {
      container: self.container,
      account: self.account,
      accessKey: self.accessKey,
    };
  },
};
