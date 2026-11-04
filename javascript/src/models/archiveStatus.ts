// this file is @generated

export enum ArchiveStatus {
  Active = "active",
  Archived = "archived",
  All = "all",
}

export const ArchiveStatusSerializer = {
  _fromJsonObject(object: any): ArchiveStatus {
    return object;
  },

  _toJsonObject(self: ArchiveStatus): any {
    return self;
  },
};
