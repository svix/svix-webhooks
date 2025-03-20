// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { BorderRadiusEnum, BorderRadiusEnumSerializer } from "./borderRadiusEnum";

export interface BorderRadiusConfig {
  button?: BorderRadiusEnum | null;
  card?: BorderRadiusEnum | null;
  input?: BorderRadiusEnum | null;
}

export const BorderRadiusConfigSerializer = {
  _fromJsonObject(object: any): BorderRadiusConfig {
    return {
      button: object["button"]
        ? BorderRadiusEnumSerializer._fromJsonObject(object["button"])
        : undefined,
      card: object["card"]
        ? BorderRadiusEnumSerializer._fromJsonObject(object["card"])
        : undefined,
      input: object["input"]
        ? BorderRadiusEnumSerializer._fromJsonObject(object["input"])
        : undefined,
    };
  },

  _toJsonObject(self: BorderRadiusConfig): any {
    return {
      button: self.button
        ? BorderRadiusEnumSerializer._toJsonObject(self.button)
        : undefined,
      card: self.card ? BorderRadiusEnumSerializer._toJsonObject(self.card) : undefined,
      input: self.input
        ? BorderRadiusEnumSerializer._toJsonObject(self.input)
        : undefined,
    };
  },
};
