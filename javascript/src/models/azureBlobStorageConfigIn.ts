// this file is @generated

export interface AzureBlobStorageConfigIn {
  container: string;
  account: string;
  accessKey: string;
}

export const AzureBlobStorageConfigInSerializer = {
  _fromJsonObject(object: any): AzureBlobStorageConfigIn {
    return {
      container: object["container"],
      account: object["account"],
      accessKey: object["accessKey"],
    };
  },

  _toJsonObject(self: AzureBlobStorageConfigIn): any {
    return {
      container: self.container,
      account: self.account,
      accessKey: self.accessKey,
    };
  },
};
