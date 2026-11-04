// this file is @generated

export interface RedshiftConfigPatch {
  accessKeyId?: string;
  secretAccessKey?: string;
  roleArn?: string;
  externalId?: string;
  region?: string;
  /**
   * Database name.
   *
   * Only required if not using transformations.
   */
  dbName?: string;
  /**
   * Schema name.
   *
   * Only used if not using transformations.
   */
  schemaName?: string | null;
  /**
   * Table name.
   *
   * Only required if not using transformations.
   */
  tableName?: string;
}

export const RedshiftConfigPatchSerializer = {
  _fromJsonObject(object: any): RedshiftConfigPatch {
    return {
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      roleArn: object["roleArn"],
      externalId: object["externalId"],
      region: object["region"],
      dbName: object["dbName"],
      schemaName: object["schemaName"],
      tableName: object["tableName"],
    };
  },

  _toJsonObject(self: RedshiftConfigPatch): any {
    return {
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
      roleArn: self.roleArn,
      externalId: self.externalId,
      region: self.region,
      dbName: self.dbName,
      schemaName: self.schemaName,
      tableName: self.tableName,
    };
  },
};
