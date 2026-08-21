// this file is @generated

/** Configuration for a Google Cloud BigQuery sink. */
export interface BigQueryConfigIn {
  projectId: string;
  datasetId: string;
  tableId: string;
  /** Google Cloud Credentials JSON Object as a string. */
  credentials: string;
}

export const BigQueryConfigInSerializer = {
  _fromJsonObject(object: any): BigQueryConfigIn {
    return {
      projectId: object["projectId"],
      datasetId: object["datasetId"],
      tableId: object["tableId"],
      credentials: object["credentials"],
    };
  },

  _toJsonObject(self: BigQueryConfigIn): any {
    return {
      projectId: self.projectId,
      datasetId: self.datasetId,
      tableId: self.tableId,
      credentials: self.credentials,
    };
  },
};
