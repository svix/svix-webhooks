// this file is @generated

export interface AzureBlobStorageConfigOut {
  container: string;
  account: string;
}

export const AzureBlobStorageConfigOutSerializer = {
  _fromJsonObject(object: any): AzureBlobStorageConfigOut {
    return {
      container: object["container"],
      account: object["account"],
    };
  },

  _toJsonObject(self: AzureBlobStorageConfigOut): any {
    return {
      container: self.container,
      account: self.account,
    };
  },
};
