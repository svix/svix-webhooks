// this file is @generated

export interface RedshiftConfigOut {
  accessKeyId: string;
  region: string;
  clusterIdentifier?: string | null;
  dbUser?: string | null;
  workgroupName?: string | null;
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

export const RedshiftConfigOutSerializer = {
  _fromJsonObject(object: any): RedshiftConfigOut {
    return {
      accessKeyId: object["accessKeyId"],
      region: object["region"],
      clusterIdentifier: object["clusterIdentifier"],
      dbUser: object["dbUser"],
      workgroupName: object["workgroupName"],
      dbName: object["dbName"],
      schemaName: object["schemaName"],
      tableName: object["tableName"],
    };
  },

  _toJsonObject(self: RedshiftConfigOut): any {
    return {
      accessKeyId: self.accessKeyId,
      region: self.region,
      clusterIdentifier: self.clusterIdentifier,
      dbUser: self.dbUser,
      workgroupName: self.workgroupName,
      dbName: self.dbName,
      schemaName: self.schemaName,
      tableName: self.tableName,
    };
  },
};
