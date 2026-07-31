// this file is @generated

/** Configuration for a Google Cloud BigQuery sink. */
export interface BigQueryConfig {
  projectId: string;
  datasetId: string;
  tableId: string;
  /** Google Cloud Credentials JSON Object as a string. */
  credentials: string;
}

export const BigQueryConfigSerializer = {
  _fromJsonObject(object: any): BigQueryConfig {
    return {
      projectId: object["projectId"],
      datasetId: object["datasetId"],
      tableId: object["tableId"],
      credentials: object["credentials"],
    };
  },

  _toJsonObject(self: BigQueryConfig): any {
    return {
      projectId: self.projectId,
      datasetId: self.datasetId,
      tableId: self.tableId,
      credentials: self.credentials,
    };
  },
};
