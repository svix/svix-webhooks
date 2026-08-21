// this file is @generated

export interface AzureBlobStorageConfigPatch {
  container?: string;
  account?: string;
  accessKey?: string;
}

export const AzureBlobStorageConfigPatchSerializer = {
  _fromJsonObject(object: any): AzureBlobStorageConfigPatch {
    return {
      container: object["container"],
      account: object["account"],
      accessKey: object["accessKey"],
    };
  },

  _toJsonObject(self: AzureBlobStorageConfigPatch): any {
    return {
      container: self.container,
      account: self.account,
      accessKey: self.accessKey,
    };
  },
};
