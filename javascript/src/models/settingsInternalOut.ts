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

export interface SettingsInternalOut {
  colorPaletteDark?: CustomColorPalette | null;
  colorPaletteLight?: CustomColorPalette | null;
  customBaseFontSize?: number | null;
  customColor?: string | null;
  customFontFamily?: string | null;
  customFontFamilyUrl?: string | null;
  customLogoUrl?: string | null;
  customStringsOverride?: CustomStringsOverride | null;
  customThemeOverride?: CustomThemeOverride | null;
  disableEndpointOnFailure?: boolean;
  displayName?: string | null;
  enableChannels?: boolean;
  enableEndpointMtlsConfig?: boolean;
  enableEndpointOauthConfig?: boolean;
  enableIntegrationManagement?: boolean;
  enableMessageStream?: boolean;
  enableMsgAtmptLog?: boolean;
  enableOtlp?: boolean;
  enableTransformations?: boolean;
  enforceHttps?: boolean;
  eventCatalogPublished?: boolean;
  readOnly?: boolean;
  requireEndpointChannel?: boolean;
  retryPolicy?: number[] | null;
  showUseSvixPlay?: boolean;
  whitelabelHeaders?: boolean;
  wipeSuccessfulPayload?: boolean;
}

export const SettingsInternalOutSerializer = {
  _fromJsonObject(object: any): SettingsInternalOut {
    return {
      colorPaletteDark: object["colorPaletteDark"]
        ? CustomColorPaletteSerializer._fromJsonObject(object["colorPaletteDark"])
        : undefined,
      colorPaletteLight: object["colorPaletteLight"]
        ? CustomColorPaletteSerializer._fromJsonObject(object["colorPaletteLight"])
        : undefined,
      customBaseFontSize: object["customBaseFontSize"],
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
      disableEndpointOnFailure: object["disableEndpointOnFailure"],
      displayName: object["displayName"],
      enableChannels: object["enableChannels"],
      enableEndpointMtlsConfig: object["enableEndpointMtlsConfig"],
      enableEndpointOauthConfig: object["enableEndpointOauthConfig"],
      enableIntegrationManagement: object["enableIntegrationManagement"],
      enableMessageStream: object["enableMessageStream"],
      enableMsgAtmptLog: object["enableMsgAtmptLog"],
      enableOtlp: object["enableOtlp"],
      enableTransformations: object["enableTransformations"],
      enforceHttps: object["enforceHttps"],
      eventCatalogPublished: object["eventCatalogPublished"],
      readOnly: object["readOnly"],
      requireEndpointChannel: object["requireEndpointChannel"],
      retryPolicy: object["retryPolicy"],
      showUseSvixPlay: object["showUseSvixPlay"],
      whitelabelHeaders: object["whitelabelHeaders"],
      wipeSuccessfulPayload: object["wipeSuccessfulPayload"],
    };
  },

  _toJsonObject(self: SettingsInternalOut): any {
    return {
      colorPaletteDark: self.colorPaletteDark
        ? CustomColorPaletteSerializer._toJsonObject(self.colorPaletteDark)
        : undefined,
      colorPaletteLight: self.colorPaletteLight
        ? CustomColorPaletteSerializer._toJsonObject(self.colorPaletteLight)
        : undefined,
      customBaseFontSize: self.customBaseFontSize,
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
      disableEndpointOnFailure: self.disableEndpointOnFailure,
      displayName: self.displayName,
      enableChannels: self.enableChannels,
      enableEndpointMtlsConfig: self.enableEndpointMtlsConfig,
      enableEndpointOauthConfig: self.enableEndpointOauthConfig,
      enableIntegrationManagement: self.enableIntegrationManagement,
      enableMessageStream: self.enableMessageStream,
      enableMsgAtmptLog: self.enableMsgAtmptLog,
      enableOtlp: self.enableOtlp,
      enableTransformations: self.enableTransformations,
      enforceHttps: self.enforceHttps,
      eventCatalogPublished: self.eventCatalogPublished,
      readOnly: self.readOnly,
      requireEndpointChannel: self.requireEndpointChannel,
      retryPolicy: self.retryPolicy,
      showUseSvixPlay: self.showUseSvixPlay,
      whitelabelHeaders: self.whitelabelHeaders,
      wipeSuccessfulPayload: self.wipeSuccessfulPayload,
    };
  },
};
