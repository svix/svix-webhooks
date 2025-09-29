// this file is @generated
import {
  type BackgroundTaskFinishedEvent2,
  BackgroundTaskFinishedEvent2Serializer,
} from "./backgroundTaskFinishedEvent2";

/** Sent when a background task is finished. */
export interface BackgroundTaskFinishedEvent {
  data: BackgroundTaskFinishedEvent2;
  type: string;
}

export const BackgroundTaskFinishedEventSerializer = {
  _fromJsonObject(object: any): BackgroundTaskFinishedEvent {
    return {
      data: BackgroundTaskFinishedEvent2Serializer._fromJsonObject(object["data"]),
      type: object["type"],
    };
  },

  _toJsonObject(self: BackgroundTaskFinishedEvent): any {
    return {
      data: BackgroundTaskFinishedEvent2Serializer._toJsonObject(self.data),
      type: self.type,
    };
  },
};
