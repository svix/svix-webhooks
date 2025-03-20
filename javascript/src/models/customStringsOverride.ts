// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface CustomStringsOverride {
  channelsHelp?: string | null;
  channelsMany?: string | null;
  channelsOne?: string | null;
}

export const CustomStringsOverrideSerializer = {
  _fromJsonObject(object: any): CustomStringsOverride {
    return {
      channelsHelp: object["channelsHelp"],
      channelsMany: object["channelsMany"],
      channelsOne: object["channelsOne"],
    };
  },

  _toJsonObject(self: CustomStringsOverride): any {
    return {
      channelsHelp: self.channelsHelp,
      channelsMany: self.channelsMany,
      channelsOne: self.channelsOne,
    };
  },
};
