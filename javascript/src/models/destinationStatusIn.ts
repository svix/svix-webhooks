// this file is @generated

export enum DestinationStatusIn {
  Enabled = "enabled",
  Disabled = "disabled",
}

export const DestinationStatusInSerializer = {
  _fromJsonObject(object: any): DestinationStatusIn {
    return object;
  },

  _toJsonObject(self: DestinationStatusIn): any {
    return self;
  },
};
