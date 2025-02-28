// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  BackgroundTaskStatus,
  BackgroundTaskStatusSerializer,
} from "./backgroundTaskStatus";
import { BackgroundTaskType, BackgroundTaskTypeSerializer } from "./backgroundTaskType";

export interface ExpungeAllContentsOut {
  /** The QueueBackgroundTask's ID. */
  id: string;
  status: BackgroundTaskStatus;
  task: BackgroundTaskType;
}

export const ExpungeAllContentsOutSerializer = {
  _fromJsonObject(object: any): ExpungeAllContentsOut {
    return {
      id: object["id"],
      status: BackgroundTaskStatusSerializer._fromJsonObject(object["status"]),
      task: BackgroundTaskTypeSerializer._fromJsonObject(object["task"]),
    };
  },

  _toJsonObject(self: ExpungeAllContentsOut): any {
    return {
      id: self.id,
      status: BackgroundTaskStatusSerializer._toJsonObject(self.status),
      task: BackgroundTaskTypeSerializer._toJsonObject(self.task),
    };
  },
};
