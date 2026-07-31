// this file is @generated
import {
  type BackgroundTaskStatus,
  BackgroundTaskStatusSerializer,
} from "./backgroundTaskStatus";
import {
  type BackgroundTaskType,
  BackgroundTaskTypeSerializer,
} from "./backgroundTaskType";

export interface AppUsageStatsOut {
  /**
   * Any app IDs or UIDs received in the request that weren't found.
   *
   * Stats will be produced for all the others.
   */
  unresolvedAppIds: string[];
  /** The QueueBackgroundTask's ID. */
  id: string;
  status: BackgroundTaskStatus;
  task: BackgroundTaskType;
  updatedAt: Date;
}

export const AppUsageStatsOutSerializer = {
  _fromJsonObject(object: any): AppUsageStatsOut {
    return {
      unresolvedAppIds: object["unresolvedAppIds"],
      id: object["id"],
      status: BackgroundTaskStatusSerializer._fromJsonObject(object["status"]),
      task: BackgroundTaskTypeSerializer._fromJsonObject(object["task"]),
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: AppUsageStatsOut): any {
    return {
      unresolvedAppIds: self.unresolvedAppIds,
      id: self.id,
      status: BackgroundTaskStatusSerializer._toJsonObject(self.status),
      task: BackgroundTaskTypeSerializer._toJsonObject(self.task),
      updatedAt: self.updatedAt,
    };
  },
};
