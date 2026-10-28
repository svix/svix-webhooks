// this file is @generated
/**
 * Operation for bulk indexing.
 *
 * `index` will do an upsert for the given document ID, and `create` will insert it
 * (and do nothing on conflict). Only `create` is supported for Data Stream targets.
 */
export enum ElasticSearchOperation {
  Index = "index",
  Create = "create",
}

export const ElasticSearchOperationSerializer = {
  _fromJsonObject(object: any): ElasticSearchOperation {
    return object;
  },

  _toJsonObject(self: ElasticSearchOperation): any {
    return self;
  },
};
