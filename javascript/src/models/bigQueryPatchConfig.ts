// this file is @generated

export interface BigQueryPatchConfig {
  credentials?: string;
  datasetId?: string;
  projectId?: string;
  tableId?: string;
}

export const BigQueryPatchConfigSerializer = {
  _fromJsonObject(object: any): BigQueryPatchConfig {
    return {
      credentials: object["credentials"],
      datasetId: object["datasetId"],
      projectId: object["projectId"],
      tableId: object["tableId"],
    };
  },

  _toJsonObject(self: BigQueryPatchConfig): any {
    return {
      credentials: self.credentials,
      datasetId: self.datasetId,
      projectId: self.projectId,
      tableId: self.tableId,
    };
  },
};
