// this file is @generated

export interface SlackConfig {
  secret: string;
}

export const SlackConfigSerializer = {
  _fromJsonObject(object: any): SlackConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: SlackConfig): any {
    return {
      secret: self.secret,
    };
  },
};
