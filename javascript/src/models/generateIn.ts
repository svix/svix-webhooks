// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface GenerateIn {
  prompt: string;
}

export const GenerateInSerializer = {
  _fromJsonObject(object: any): GenerateIn {
    return {
      prompt: object["prompt"],
    };
  },

  _toJsonObject(self: GenerateIn): any {
    return {
      prompt: self.prompt,
    };
  },
};
