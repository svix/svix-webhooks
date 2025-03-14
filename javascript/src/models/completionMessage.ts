// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface CompletionMessage {
  content: string;
  role: string;
}

export const CompletionMessageSerializer = {
  _fromJsonObject(object: any): CompletionMessage {
    return {
      content: object["content"],
      role: object["role"],
    };
  },

  _toJsonObject(self: CompletionMessage): any {
    return {
      content: self.content,
      role: self.role,
    };
  },
};
