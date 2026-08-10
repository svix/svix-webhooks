// this file is @generated

export interface BigQueryConfigOut {
  projectId: string;
  datasetId: string;
  tableId: string;
}

export const BigQueryConfigOutSerializer = {
  _fromJsonObject(object: any): BigQueryConfigOut {
    return {
      projectId: object["projectId"],
      datasetId: object["datasetId"],
      tableId: object["tableId"],
    };
  },

  _toJsonObject(self: BigQueryConfigOut): any {
    return {
      projectId: self.projectId,
      datasetId: self.datasetId,
      tableId: self.tableId,
    };
  },
};
