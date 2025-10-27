// this file is @generated

import {
  type AggregateEventTypesOut,
  AggregateEventTypesOutSerializer,
} from "../models/aggregateEventTypesOut";
import {
  type AppUsageStatsIn,
  AppUsageStatsInSerializer,
} from "../models/appUsageStatsIn";
import {
  type AppUsageStatsOut,
  AppUsageStatsOutSerializer,
} from "../models/appUsageStatsOut";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export interface StatisticsAggregateAppStatsOptions {
  idempotencyKey?: string;
}

export class Statistics {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /**
   * Creates a background task to calculate the message destinations for all applications in the environment.
   *
   * Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
   * retrieve the results of the operation.
   *
   * The completed background task will return a payload like the following:
   * ```json
   * {
   *   "id": "qtask_33qe39Stble9Rn3ZxFrqL5ZSsjT",
   *   "status": "finished",
   *   "task": "application.stats",
   *   "data": {
   *     "appStats": [
   *       {
   *         "messageDestinations": 2,
   *         "appId": "app_33W1An2Zz5cO9SWbhHsYyDmVC6m",
   *         "appUid": null
   *       }
   *     ]
   *   }
   * }
   * ```
   */
  public aggregateAppStats(
    appUsageStatsIn: AppUsageStatsIn,
    options?: StatisticsAggregateAppStatsOptions
  ): Promise<AppUsageStatsOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/stats/usage/app");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(AppUsageStatsInSerializer._toJsonObject(appUsageStatsIn));

    return request.send(this.requestCtx, AppUsageStatsOutSerializer._fromJsonObject);
  }

  /**
   * Creates a background task to calculate the listed event types for all apps in the organization.
   *
   * Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
   * retrieve the results of the operation.
   *
   * The completed background task will return a payload like the following:
   * ```json
   * {
   *   "id": "qtask_33qe39Stble9Rn3ZxFrqL5ZSsjT",
   *   "status": "finished",
   *   "task": "event-type.aggregate",
   *   "data": {
   *     "event_types": [
   *       {
   *         "appId": "app_33W1An2Zz5cO9SWbhHsYyDmVC6m",
   *         "explicitlySubscribedEventTypes": ["user.signup", "user.deleted"],
   *         "hasCatchAllEndpoint": false
   *       }
   *     ]
   *   }
   * }
   * ```
   */
  public aggregateEventTypes(): Promise<AggregateEventTypesOut> {
    const request = new SvixRequest(HttpMethod.PUT, "/api/v1/stats/usage/event-types");

    return request.send(
      this.requestCtx,
      AggregateEventTypesOutSerializer._fromJsonObject
    );
  }
}
