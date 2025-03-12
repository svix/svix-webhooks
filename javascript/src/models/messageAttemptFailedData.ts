// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface MessageAttemptFailedData {
  /** The MessageAttempt's ID. */
  id: string;
  responseStatusCode: number;
  timestamp: Date;
}

export const MessageAttemptFailedDataSerializer = {
  _fromJsonObject(object: any): MessageAttemptFailedData {
    return {
      id: object["id"],
      responseStatusCode: object["responseStatusCode"],
      timestamp: new Date(object["timestamp"]),
    };
  },

  _toJsonObject(self: MessageAttemptFailedData): any {
    return {
      id: self.id,
      responseStatusCode: self.responseStatusCode,
      timestamp: self.timestamp,
    };
  },
};
