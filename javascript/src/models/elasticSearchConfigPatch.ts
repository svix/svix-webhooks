// this file is @generated
import {
  type ElasticSearchOperation,
  ElasticSearchOperationSerializer,
} from "./elasticSearchOperation";

export interface ElasticSearchConfigPatch {
  indexName?: string;
  url?: string;
  apiKey?: string;
  refresh?: boolean;
  operation?: ElasticSearchOperation;
}

export const ElasticSearchConfigPatchSerializer = {
  _fromJsonObject(object: any): ElasticSearchConfigPatch {
    return {
      indexName: object["indexName"],
      url: object["url"],
      apiKey: object["apiKey"],
      refresh: object["refresh"],
      operation:
        object["operation"] != null
          ? ElasticSearchOperationSerializer._fromJsonObject(object["operation"])
          : undefined,
    };
  },

  _toJsonObject(self: ElasticSearchConfigPatch): any {
    return {
      indexName: self.indexName,
      url: self.url,
      apiKey: self.apiKey,
      refresh: self.refresh,
      operation:
        self.operation != null
          ? ElasticSearchOperationSerializer._toJsonObject(self.operation)
          : undefined,
    };
  },
};
