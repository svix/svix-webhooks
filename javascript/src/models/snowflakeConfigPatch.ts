// this file is @generated

export interface SnowflakeConfigPatch {
  privateKey?: string;
  accountIdentifier?: string;
  userId?: string;
  /**
   * Database name.
   *
   * Only required if not using transformations.
   */
  dbName?: string;
  /**
   * Schema name.
   *
   * Only required if not using transformations.
   */
  schemaName?: string;
  /**
   * Table name.
   *
   * Only required if not using transformations.
   */
  tableName?: string;
}

export const SnowflakeConfigPatchSerializer = {
  _fromJsonObject(object: any): SnowflakeConfigPatch {
    return {
      privateKey: object["privateKey"],
      accountIdentifier: object["accountIdentifier"],
      userId: object["userId"],
      dbName: object["dbName"],
      schemaName: object["schemaName"],
      tableName: object["tableName"],
    };
  },

  _toJsonObject(self: SnowflakeConfigPatch): any {
    return {
      privateKey: self.privateKey,
      accountIdentifier: self.accountIdentifier,
      userId: self.userId,
      dbName: self.dbName,
      schemaName: self.schemaName,
      tableName: self.tableName,
    };
  },
};
