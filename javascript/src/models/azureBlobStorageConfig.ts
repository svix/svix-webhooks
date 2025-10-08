// this file is @generated

export interface AzureBlobStorageConfig {
  accessKey: string;
  account: string;
  container: string;
}

export const AzureBlobStorageConfigSerializer = {
  _fromJsonObject(object: any): AzureBlobStorageConfig {
    return {
      accessKey: object["accessKey"],
      account: object["account"],
      container: object["container"],
    };
  },

  _toJsonObject(self: AzureBlobStorageConfig): any {
    return {
      accessKey: self.accessKey,
      account: self.account,
      container: self.container,
    };
  },
};
