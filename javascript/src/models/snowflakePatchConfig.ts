// this file is @generated

export interface SnowflakePatchConfig {
  accountIdentifier?: string;
  /**
   * Database name.
   *
   * Only required if not using transformations.
   */
  dbName?: string;
  privateKey?: string;
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
  userId?: string;
}

export const SnowflakePatchConfigSerializer = {
  _fromJsonObject(object: any): SnowflakePatchConfig {
    return {
      accountIdentifier: object["accountIdentifier"],
      dbName: object["dbName"],
      privateKey: object["privateKey"],
      schemaName: object["schemaName"],
      tableName: object["tableName"],
      userId: object["userId"],
    };
  },

  _toJsonObject(self: SnowflakePatchConfig): any {
    return {
      accountIdentifier: self.accountIdentifier,
      dbName: self.dbName,
      privateKey: self.privateKey,
      schemaName: self.schemaName,
      tableName: self.tableName,
      userId: self.userId,
    };
  },
};
