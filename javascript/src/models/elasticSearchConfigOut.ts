// this file is @generated

export interface ElasticSearchConfigOut {
  indexName: string;
  url: string;
  refresh: boolean;
}

export const ElasticSearchConfigOutSerializer = {
  _fromJsonObject(object: any): ElasticSearchConfigOut {
    return {
      indexName: object["indexName"],
      url: object["url"],
      refresh: object["refresh"],
    };
  },

  _toJsonObject(self: ElasticSearchConfigOut): any {
    return {
      indexName: self.indexName,
      url: self.url,
      refresh: self.refresh,
    };
  },
};
