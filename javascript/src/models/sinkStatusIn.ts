// this file is @generated

export enum SinkStatusIn {
  Enabled = "enabled",
  Disabled = "disabled",
}

export const SinkStatusInSerializer = {
  _fromJsonObject(object: any): SinkStatusIn {
    return object;
  },

  _toJsonObject(self: SinkStatusIn): any {
    return self;
  },
};
