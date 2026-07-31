// this file is @generated

/**
 * Configuration parameters for defining a Redshift sink.
 *
 * For provisioned clusters, set `cluster_identifier` and `db_user`. For Redshift Serverless, set `workgroup_name`.
 */
export interface RedshiftConfig {
  accessKeyId: string;
  secretAccessKey: string;
  region: string;
  /** Required for provisioned clusters. */
  clusterIdentifier?: string | null;
  /** Required for provisioned clusters. */
  dbUser?: string | null;
  /** Required for Redshift Serverless. */
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

export const RedshiftConfigSerializer = {
  _fromJsonObject(object: any): RedshiftConfig {
    return {
      accessKeyId: object["accessKeyId"],
      secretAccessKey: object["secretAccessKey"],
      region: object["region"],
      clusterIdentifier: object["clusterIdentifier"],
      dbUser: object["dbUser"],
      workgroupName: object["workgroupName"],
      dbName: object["dbName"],
      schemaName: object["schemaName"],
      tableName: object["tableName"],
    };
  },

  _toJsonObject(self: RedshiftConfig): any {
    return {
      accessKeyId: self.accessKeyId,
      secretAccessKey: self.secretAccessKey,
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
