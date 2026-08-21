// this file is @generated

export interface SnowflakeConfigOut {
  accountIdentifier: string;
  userId: string;
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

export const SnowflakeConfigOutSerializer = {
  _fromJsonObject(object: any): SnowflakeConfigOut {
    return {
      accountIdentifier: object["accountIdentifier"],
      userId: object["userId"],
      dbName: object["dbName"],
      schemaName: object["schemaName"],
      tableName: object["tableName"],
    };
  },

  _toJsonObject(self: SnowflakeConfigOut): any {
    return {
      accountIdentifier: self.accountIdentifier,
      userId: self.userId,
      dbName: self.dbName,
      schemaName: self.schemaName,
      tableName: self.tableName,
    };
  },
};
