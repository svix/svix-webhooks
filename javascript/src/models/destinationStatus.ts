// this file is @generated

export enum DestinationStatus {
  Enabled = "enabled",
  Paused = "paused",
  Disabled = "disabled",
  Retrying = "retrying",
}

export const DestinationStatusSerializer = {
  _fromJsonObject(object: any): DestinationStatus {
    return object;
  },

  _toJsonObject(self: DestinationStatus): any {
    return self;
  },
};
