// this file is @generated
import { ApplicationStats, ApplicationStatsSerializer } from "../models/applicationStats";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface ApplicationGetStatsOptions {
  /** Filter the range to data starting from this date. */
  since: Date | null;
  /** Filter the range to data ending by this date. */
  until: Date | null;
}

export class Application {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Get basic statistics for the application. */
  public getStats(
    appId: string,
    options: ApplicationGetStatsOptions
  ): Promise<ApplicationStats> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/app/{app_id}/stats");

    request.setPathParam("app_id", appId);
    request.setQueryParam("since", options.since);
    request.setQueryParam("until", options.until);

    return request.send(this.requestCtx, ApplicationStatsSerializer._fromJsonObject);
  }
}
