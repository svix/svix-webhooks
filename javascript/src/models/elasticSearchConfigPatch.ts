// this file is @generated

export interface ElasticSearchConfigPatch {
  indexName?: string;
  url?: string;
  apiKey?: string;
  refresh?: boolean;
}

export const ElasticSearchConfigPatchSerializer = {
  _fromJsonObject(object: any): ElasticSearchConfigPatch {
    return {
      indexName: object["indexName"],
      url: object["url"],
      apiKey: object["apiKey"],
      refresh: object["refresh"],
    };
  },

  _toJsonObject(self: ElasticSearchConfigPatch): any {
    return {
      indexName: self.indexName,
      url: self.url,
      apiKey: self.apiKey,
      refresh: self.refresh,
    };
  },
};
