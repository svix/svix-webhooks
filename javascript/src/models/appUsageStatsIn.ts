// this file is @generated

export interface AppUsageStatsIn {
  since: Date;
  until: Date;
  /**
   * Specific app IDs or UIDs to aggregate stats for.
   *
   * Note that if none of the given IDs or UIDs are resolved, a 422 response will be given.
   */
  appIds?: string[] | null;
}

export const AppUsageStatsInSerializer = {
  _fromJsonObject(object: any): AppUsageStatsIn {
    return {
      since: new Date(object["since"]),
      until: new Date(object["until"]),
      appIds: object["appIds"],
    };
  },

  _toJsonObject(self: AppUsageStatsIn): any {
    return {
      since: self.since,
      until: self.until,
      appIds: self.appIds,
    };
  },
};
