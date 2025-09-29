// this file is @generated
import {
  type BackgroundTaskStatus,
  BackgroundTaskStatusSerializer,
} from "./backgroundTaskStatus";
import {
  type BackgroundTaskType,
  BackgroundTaskTypeSerializer,
} from "./backgroundTaskType";

export interface RecoverOut {
  /** The QueueBackgroundTask's ID. */
  id: string;
  status: BackgroundTaskStatus;
  task: BackgroundTaskType;
}

export const RecoverOutSerializer = {
  _fromJsonObject(object: any): RecoverOut {
    return {
      id: object["id"],
      status: BackgroundTaskStatusSerializer._fromJsonObject(object["status"]),
      task: BackgroundTaskTypeSerializer._fromJsonObject(object["task"]),
    };
  },

  _toJsonObject(self: RecoverOut): any {
    return {
      id: self.id,
      status: BackgroundTaskStatusSerializer._toJsonObject(self.status),
      task: BackgroundTaskTypeSerializer._toJsonObject(self.task),
    };
  },
};
