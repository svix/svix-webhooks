// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface CustomColorPalette {
  backgroundHover?: string | null;
  backgroundPrimary?: string | null;
  backgroundSecondary?: string | null;
  buttonPrimary?: string | null;
  interactiveAccent?: string | null;
  navigationAccent?: string | null;
  primary?: string | null;
  textDanger?: string | null;
  textPrimary?: string | null;
}

export const CustomColorPaletteSerializer = {
  _fromJsonObject(object: any): CustomColorPalette {
    return {
      backgroundHover: object["backgroundHover"],
      backgroundPrimary: object["backgroundPrimary"],
      backgroundSecondary: object["backgroundSecondary"],
      buttonPrimary: object["buttonPrimary"],
      interactiveAccent: object["interactiveAccent"],
      navigationAccent: object["navigationAccent"],
      primary: object["primary"],
      textDanger: object["textDanger"],
      textPrimary: object["textPrimary"],
    };
  },

  _toJsonObject(self: CustomColorPalette): any {
    return {
      backgroundHover: self.backgroundHover,
      backgroundPrimary: self.backgroundPrimary,
      backgroundSecondary: self.backgroundSecondary,
      buttonPrimary: self.buttonPrimary,
      interactiveAccent: self.interactiveAccent,
      navigationAccent: self.navigationAccent,
      primary: self.primary,
      textDanger: self.textDanger,
      textPrimary: self.textPrimary,
    };
  },
};
