// this file is @generated

export enum SinkStatus {
  Enabled = "enabled",
  Paused = "paused",
  Disabled = "disabled",
  Retrying = "retrying",
}

export const SinkStatusSerializer = {
  _fromJsonObject(object: any): SinkStatus {
    return object;
  },

  _toJsonObject(self: SinkStatus): any {
    return self;
  },
};
