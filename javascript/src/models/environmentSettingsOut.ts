// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { CustomColorPalette, CustomColorPaletteSerializer } from "./customColorPalette";
import {
  CustomStringsOverride,
  CustomStringsOverrideSerializer,
} from "./customStringsOverride";
import {
  CustomThemeOverride,
  CustomThemeOverrideSerializer,
} from "./customThemeOverride";

export interface EnvironmentSettingsOut {
  colorPaletteDark?: CustomColorPalette | null;
  colorPaletteLight?: CustomColorPalette | null;
  customColor?: string | null;
  customFontFamily?: string | null;
  customFontFamilyUrl?: string | null;
  customLogoUrl?: string | null;
  customStringsOverride?: CustomStringsOverride | null;
  customThemeOverride?: CustomThemeOverride | null;
  displayName?: string | null;
  enableChannels?: boolean;
  enableEndpointMtlsConfig?: boolean;
  enableEndpointOauthConfig?: boolean;
  enableIntegrationManagement?: boolean;
  enableMessageStream?: boolean;
  enableMessageTags?: boolean;
  enableTransformations?: boolean;
  showUseSvixPlay?: boolean;
  wipeSuccessfulPayload?: boolean;
}

export const EnvironmentSettingsOutSerializer = {
  _fromJsonObject(object: any): EnvironmentSettingsOut {
    return {
      colorPaletteDark: object["colorPaletteDark"]
        ? CustomColorPaletteSerializer._fromJsonObject(object["colorPaletteDark"])
        : undefined,
      colorPaletteLight: object["colorPaletteLight"]
        ? CustomColorPaletteSerializer._fromJsonObject(object["colorPaletteLight"])
        : undefined,
      customColor: object["customColor"],
      customFontFamily: object["customFontFamily"],
      customFontFamilyUrl: object["customFontFamilyUrl"],
      customLogoUrl: object["customLogoUrl"],
      customStringsOverride: object["customStringsOverride"]
        ? CustomStringsOverrideSerializer._fromJsonObject(object["customStringsOverride"])
        : undefined,
      customThemeOverride: object["customThemeOverride"]
        ? CustomThemeOverrideSerializer._fromJsonObject(object["customThemeOverride"])
        : undefined,
      displayName: object["displayName"],
      enableChannels: object["enableChannels"],
      enableEndpointMtlsConfig: object["enableEndpointMtlsConfig"],
      enableEndpointOauthConfig: object["enableEndpointOauthConfig"],
      enableIntegrationManagement: object["enableIntegrationManagement"],
      enableMessageStream: object["enableMessageStream"],
      enableMessageTags: object["enableMessageTags"],
      enableTransformations: object["enableTransformations"],
      showUseSvixPlay: object["showUseSvixPlay"],
      wipeSuccessfulPayload: object["wipeSuccessfulPayload"],
    };
  },

  _toJsonObject(self: EnvironmentSettingsOut): any {
    return {
      colorPaletteDark: self.colorPaletteDark
        ? CustomColorPaletteSerializer._toJsonObject(self.colorPaletteDark)
        : undefined,
      colorPaletteLight: self.colorPaletteLight
        ? CustomColorPaletteSerializer._toJsonObject(self.colorPaletteLight)
        : undefined,
      customColor: self.customColor,
      customFontFamily: self.customFontFamily,
      customFontFamilyUrl: self.customFontFamilyUrl,
      customLogoUrl: self.customLogoUrl,
      customStringsOverride: self.customStringsOverride
        ? CustomStringsOverrideSerializer._toJsonObject(self.customStringsOverride)
        : undefined,
      customThemeOverride: self.customThemeOverride
        ? CustomThemeOverrideSerializer._toJsonObject(self.customThemeOverride)
        : undefined,
      displayName: self.displayName,
      enableChannels: self.enableChannels,
      enableEndpointMtlsConfig: self.enableEndpointMtlsConfig,
      enableEndpointOauthConfig: self.enableEndpointOauthConfig,
      enableIntegrationManagement: self.enableIntegrationManagement,
      enableMessageStream: self.enableMessageStream,
      enableMessageTags: self.enableMessageTags,
      enableTransformations: self.enableTransformations,
      showUseSvixPlay: self.showUseSvixPlay,
      wipeSuccessfulPayload: self.wipeSuccessfulPayload,
    };
  },
};
