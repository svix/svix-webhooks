// this file is @generated

export interface AzureBlobStorageConfigIn {
  container: string;
  account: string;
  /**
   * Access key.
   *
   * Currently a required field, but marked as optional because we may add different authentication in the future.
   */
  accessKey?: string | null;
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
