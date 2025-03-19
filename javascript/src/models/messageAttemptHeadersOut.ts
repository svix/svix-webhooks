// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface MessageAttemptHeadersOut {
  responseHeaders?: string[][] | null;
  sensitive: string[];
  sentHeaders: { [key: string]: string };
}

export const MessageAttemptHeadersOutSerializer = {
  _fromJsonObject(object: any): MessageAttemptHeadersOut {
    return {
      responseHeaders: object["responseHeaders"].map((item: string[]) => item),
      sensitive: object["sensitive"],
      sentHeaders: object["sentHeaders"],
    };
  },

  _toJsonObject(self: MessageAttemptHeadersOut): any {
    return {
      responseHeaders: self.responseHeaders?.map((item) => item),
      sensitive: self.sensitive,
      sentHeaders: self.sentHeaders,
    };
  },
};
