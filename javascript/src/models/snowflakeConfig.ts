// this file is @generated

/** Configuration parameters for defining a Snowflake sink. */
export interface SnowflakeConfig {
  /**
   * PEM-encoded private key used for signing token-based requests to the Snowflake API.
   *
   * Beginning/end delimiters are not required.
   */
  privateKey: string;
  /** Snowflake account identifier, which includes both the organization and account IDs separated by a hyphen. */
  accountIdentifier: string;
  /** The Snowflake user id. */
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

export const SnowflakeConfigSerializer = {
  _fromJsonObject(object: any): SnowflakeConfig {
    return {
      privateKey: object["privateKey"],
      accountIdentifier: object["accountIdentifier"],
      userId: object["userId"],
      dbName: object["dbName"],
      schemaName: object["schemaName"],
      tableName: object["tableName"],
    };
  },

  _toJsonObject(self: SnowflakeConfig): any {
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
