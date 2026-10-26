// this file is @generated
import {
  type ElasticSearchOperation,
  ElasticSearchOperationSerializer,
} from "./elasticSearchOperation";

export interface ElasticSearchConfigOut {
  indexName: string;
  url: string;
  refresh: boolean;
  operation: ElasticSearchOperation;
}

export const ElasticSearchConfigOutSerializer = {
  _fromJsonObject(object: any): ElasticSearchConfigOut {
    return {
      indexName: object["indexName"],
      url: object["url"],
      refresh: object["refresh"],
      operation: ElasticSearchOperationSerializer._fromJsonObject(object["operation"]),
    };
  },

  _toJsonObject(self: ElasticSearchConfigOut): any {
    return {
      indexName: self.indexName,
      url: self.url,
      refresh: self.refresh,
      operation: ElasticSearchOperationSerializer._toJsonObject(self.operation),
    };
  },
};
