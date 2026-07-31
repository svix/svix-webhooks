// this file is @generated

export interface AzureBlobStorageConfig {
  container: string;
  account: string;
  accessKey: string;
}

export const AzureBlobStorageConfigSerializer = {
  _fromJsonObject(object: any): AzureBlobStorageConfig {
    return {
      container: object["container"],
      account: object["account"],
      accessKey: object["accessKey"],
    };
  },

  _toJsonObject(self: AzureBlobStorageConfig): any {
    return {
      container: self.container,
      account: self.account,
      accessKey: self.accessKey,
    };
  },
};
