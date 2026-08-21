// this file is @generated

/**
 * Configuration parameters for defining a Redshift sink.
 *
 * For provisioned clusters, set `cluster_identifier` and `db_user`. For Redshift Serverless, set `workgroup_name`.
 */
export interface RedshiftConfigIn {
  /**
   * Access key ID.
   *
   * Currently a required field, but marked as optional because we may add different authentication in the future.
   */
  accessKeyId?: string | null;
  /**
   * Secret access key.
   *
   * Currently a required field, but marked as optional because we may add different authentication in the future.
   */
  secretAccessKey?: string | null;
  /**
   * The region of the Redshift DB.
   *
   * Currently a required field, but marked as optional because we may infer it from other fields in the future.
   */
  region?: string | null;
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

export const RedshiftConfigInSerializer = {
  _fromJsonObject(object: any): RedshiftConfigIn {
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

  _toJsonObject(self: RedshiftConfigIn): any {
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
