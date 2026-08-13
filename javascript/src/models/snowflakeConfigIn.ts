// this file is @generated

/** Configuration parameters for defining a Snowflake sink. */
export interface SnowflakeConfigIn {
  /**
   * PEM-encoded private key used for signing token-based requests to the Snowflake API.
   *
   * Beginning/end delimiters are not required.
   *
   * Currently a required field, but marked as optional because we may add different authentication in the future.
   */
  privateKey?: string | null;
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

export const SnowflakeConfigInSerializer = {
  _fromJsonObject(object: any): SnowflakeConfigIn {
    return {
      privateKey: object["privateKey"],
      accountIdentifier: object["accountIdentifier"],
      userId: object["userId"],
      dbName: object["dbName"],
      schemaName: object["schemaName"],
      tableName: object["tableName"],
    };
  },

  _toJsonObject(self: SnowflakeConfigIn): any {
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
