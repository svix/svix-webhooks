// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export enum TransformationHttpMethod {
  Post = "POST",
  Put = "PUT",
  Patch = "PATCH",
}

export const TransformationHttpMethodSerializer = {
  _fromJsonObject(object: any): TransformationHttpMethod {
    return object;
  },

  _toJsonObject(self: TransformationHttpMethod): any {
    return self;
  },
};
