// this file is @generated

export interface BigQueryConfigPatch {
  projectId?: string;
  datasetId?: string;
  tableId?: string;
  credentials?: string;
}

export const BigQueryConfigPatchSerializer = {
  _fromJsonObject(object: any): BigQueryConfigPatch {
    return {
      projectId: object["projectId"],
      datasetId: object["datasetId"],
      tableId: object["tableId"],
      credentials: object["credentials"],
    };
  },

  _toJsonObject(self: BigQueryConfigPatch): any {
    return {
      projectId: self.projectId,
      datasetId: self.datasetId,
      tableId: self.tableId,
      credentials: self.credentials,
    };
  },
};
