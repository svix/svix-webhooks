// this file is @generated

export interface RedshiftPatchConfig {
  accessKeyId?: string;
  /**
   * Database name.
   *
   * Only required if not using transformations.
   */
  dbName?: string;
  region?: string;
  /**
   * Schema name.
   *
   * Only used if not using transformations.
   */
  schemaName?: string | null;
  secretAccessKey?: string;
  /**
   * Table name.
   *
   * Only required if not using transformations.
   */
  tableName?: string;
}

export const RedshiftPatchConfigSerializer = {
  _fromJsonObject(object: any): RedshiftPatchConfig {
    return {
      accessKeyId: object["accessKeyId"],
      dbName: object["dbName"],
      region: object["region"],
      schemaName: object["schemaName"],
      secretAccessKey: object["secretAccessKey"],
      tableName: object["tableName"],
    };
  },

  _toJsonObject(self: RedshiftPatchConfig): any {
    return {
      accessKeyId: self.accessKeyId,
      dbName: self.dbName,
      region: self.region,
      schemaName: self.schemaName,
      secretAccessKey: self.secretAccessKey,
      tableName: self.tableName,
    };
  },
};
