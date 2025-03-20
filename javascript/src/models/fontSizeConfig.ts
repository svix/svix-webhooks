// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface FontSizeConfig {
  base?: number | null;
}

export const FontSizeConfigSerializer = {
  _fromJsonObject(object: any): FontSizeConfig {
    return {
      base: object["base"],
    };
  },

  _toJsonObject(self: FontSizeConfig): any {
    return {
      base: self.base,
    };
  },
};
