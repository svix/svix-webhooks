// this file is @generated

/** Configuration parameters for defining a Snowflake sink. */
export interface SnowflakeConfig {
  /** Snowflake account identifier, which includes both the organization and account IDs separated by a hyphen. */
  accountIdentifier: string;
  /**
   * Database name.
   *
   * Only required if not using transformations.
   */
  dbName?: string;
  /**
   * PEM-encoded private key used for signing token-based requests to the Snowflake API.
   *
   * Beginning/end delimiters are not required.
   */
  privateKey: string;
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
  /** The Snowflake user id. */
  userId: string;
}

export const SnowflakeConfigSerializer = {
  _fromJsonObject(object: any): SnowflakeConfig {
    return {
      accountIdentifier: object["accountIdentifier"],
      dbName: object["dbName"],
      privateKey: object["privateKey"],
      schemaName: object["schemaName"],
      tableName: object["tableName"],
      userId: object["userId"],
    };
  },

  _toJsonObject(self: SnowflakeConfig): any {
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
