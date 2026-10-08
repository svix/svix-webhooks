// this file is @generated

/** Configuration parameters for defining an ElasticSearch/OpenSearch sink. */
export interface ElasticSearchConfigIn {
  /**
   * Name of the index to write to
   *
   * This can also be a data stream on sufficiently-new versions of ElasticSearch/OpenSearch
   */
  indexName: string;
  /**
   * Base URL to send indexing requests
   *
   * This should not include the /{index}/_bulk suffix.
   */
  url: string;
  /**
   * API key for authentication for ElasticSearch
   *
   * If not passed, any username:password embedded in the URL will be used. If none is passed,
   * the indexing will be done unauthenticated.
   */
  apiKey?: string | null;
  /** If true, Elasticsearch refreshes the affected shards to make this operation visible to search. */
  refresh?: boolean;
}

export const ElasticSearchConfigInSerializer = {
  _fromJsonObject(object: any): ElasticSearchConfigIn {
    return {
      indexName: object["indexName"],
      url: object["url"],
      apiKey: object["apiKey"],
      refresh: object["refresh"],
    };
  },

  _toJsonObject(self: ElasticSearchConfigIn): any {
    return {
      indexName: self.indexName,
      url: self.url,
      apiKey: self.apiKey,
      refresh: self.refresh,
    };
  },
};
