// this file is @generated

export interface SnowflakePatchConfig {
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

export const SnowflakePatchConfigSerializer = {
  _fromJsonObject(object: any): SnowflakePatchConfig {
    return {
      privateKey: object["privateKey"],
      accountIdentifier: object["accountIdentifier"],
      userId: object["userId"],
      dbName: object["dbName"],
      schemaName: object["schemaName"],
      tableName: object["tableName"],
    };
  },

  _toJsonObject(self: SnowflakePatchConfig): any {
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
