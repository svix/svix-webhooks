// this file is @generated

/**
 * Configuration parameters for defining a Redshift sink.
 *
 * For provisioned clusters, set `cluster_identifier` and `db_user`. For Redshift Serverless, set `workgroup_name`.
 */
export interface RedshiftConfig {
  accessKeyId: string;
  /** Required for provisioned clusters. */
  clusterIdentifier?: string | null;
  /**
   * Database name.
   *
   * Only required if not using transformations.
   */
  dbName?: string;
  /** Required for provisioned clusters. */
  dbUser?: string | null;
  region: string;
  /**
   * Schema name.
   *
   * Only used if not using transformations.
   */
  schemaName?: string | null;
  secretAccessKey: string;
  /**
   * Table name.
   *
   * Only required if not using transformations.
   */
  tableName?: string;
  /** Required for Redshift Serverless. */
  workgroupName?: string | null;
}

export const RedshiftConfigSerializer = {
  _fromJsonObject(object: any): RedshiftConfig {
    return {
      accessKeyId: object["accessKeyId"],
      clusterIdentifier: object["clusterIdentifier"],
      dbName: object["dbName"],
      dbUser: object["dbUser"],
      region: object["region"],
      schemaName: object["schemaName"],
      secretAccessKey: object["secretAccessKey"],
      tableName: object["tableName"],
      workgroupName: object["workgroupName"],
    };
  },

  _toJsonObject(self: RedshiftConfig): any {
    return {
      accessKeyId: self.accessKeyId,
      clusterIdentifier: self.clusterIdentifier,
      dbName: self.dbName,
      dbUser: self.dbUser,
      region: self.region,
      schemaName: self.schemaName,
      secretAccessKey: self.secretAccessKey,
      tableName: self.tableName,
      workgroupName: self.workgroupName,
    };
  },
};
