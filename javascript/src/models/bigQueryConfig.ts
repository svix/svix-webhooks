// this file is @generated

/** Configuration for a Google Cloud BigQuery sink. */
export interface BigQueryConfig {
  /** Google Cloud Credentials JSON Object as a string. */
  credentials: string;
  datasetId: string;
  projectId: string;
  tableId: string;
}

export const BigQueryConfigSerializer = {
  _fromJsonObject(object: any): BigQueryConfig {
    return {
      credentials: object["credentials"],
      datasetId: object["datasetId"],
      projectId: object["projectId"],
      tableId: object["tableId"],
    };
  },

  _toJsonObject(self: BigQueryConfig): any {
    return {
      credentials: self.credentials,
      datasetId: self.datasetId,
      projectId: self.projectId,
      tableId: self.tableId,
    };
  },
};
