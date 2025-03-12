// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  BackgroundTaskStatus,
  BackgroundTaskStatusSerializer,
} from "./backgroundTaskStatus";
import { BackgroundTaskType, BackgroundTaskTypeSerializer } from "./backgroundTaskType";

export interface BackgroundTaskFinishedEvent2 {
  data: any;
  status: BackgroundTaskStatus;
  task: BackgroundTaskType;
  /** The QueueBackgroundTask's ID. */
  taskId: string;
}

export const BackgroundTaskFinishedEvent2Serializer = {
  _fromJsonObject(object: any): BackgroundTaskFinishedEvent2 {
    return {
      data: object["data"],
      status: BackgroundTaskStatusSerializer._fromJsonObject(object["status"]),
      task: BackgroundTaskTypeSerializer._fromJsonObject(object["task"]),
      taskId: object["taskId"],
    };
  },

  _toJsonObject(self: BackgroundTaskFinishedEvent2): any {
    return {
      data: self.data,
      status: BackgroundTaskStatusSerializer._toJsonObject(self.status),
      task: BackgroundTaskTypeSerializer._toJsonObject(self.task),
      taskId: self.taskId,
    };
  },
};
