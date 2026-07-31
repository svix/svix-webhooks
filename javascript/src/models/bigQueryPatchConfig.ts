// this file is @generated

export interface BigQueryPatchConfig {
  projectId?: string;
  datasetId?: string;
  tableId?: string;
  credentials?: string;
}

export const BigQueryPatchConfigSerializer = {
  _fromJsonObject(object: any): BigQueryPatchConfig {
    return {
      projectId: object["projectId"],
      datasetId: object["datasetId"],
      tableId: object["tableId"],
      credentials: object["credentials"],
    };
  },

  _toJsonObject(self: BigQueryPatchConfig): any {
    return {
      projectId: self.projectId,
      datasetId: self.datasetId,
      tableId: self.tableId,
      credentials: self.credentials,
    };
  },
};
