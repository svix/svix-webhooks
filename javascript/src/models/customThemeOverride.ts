// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { BorderRadiusConfig, BorderRadiusConfigSerializer } from "./borderRadiusConfig";
import { FontSizeConfig, FontSizeConfigSerializer } from "./fontSizeConfig";

export interface CustomThemeOverride {
  borderRadius?: BorderRadiusConfig | null;
  fontSize?: FontSizeConfig | null;
}

export const CustomThemeOverrideSerializer = {
  _fromJsonObject(object: any): CustomThemeOverride {
    return {
      borderRadius: object["borderRadius"]
        ? BorderRadiusConfigSerializer._fromJsonObject(object["borderRadius"])
        : undefined,
      fontSize: object["fontSize"]
        ? FontSizeConfigSerializer._fromJsonObject(object["fontSize"])
        : undefined,
    };
  },

  _toJsonObject(self: CustomThemeOverride): any {
    return {
      borderRadius: self.borderRadius
        ? BorderRadiusConfigSerializer._toJsonObject(self.borderRadius)
        : undefined,
      fontSize: self.fontSize
        ? FontSizeConfigSerializer._toJsonObject(self.fontSize)
        : undefined,
    };
  },
};
