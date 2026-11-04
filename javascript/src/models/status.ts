// this file is @generated

export enum Status {
  Pending = "pending",
  Active = "active",
}

export const StatusSerializer = {
  _fromJsonObject(object: any): Status {
    return object;
  },

  _toJsonObject(self: Status): any {
    return self;
  },
};
