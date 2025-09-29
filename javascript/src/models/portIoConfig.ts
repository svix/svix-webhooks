// this file is @generated

export interface PortIoConfig {
  secret: string;
}

export const PortIoConfigSerializer = {
  _fromJsonObject(object: any): PortIoConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: PortIoConfig): any {
    return {
      secret: self.secret,
    };
  },
};
