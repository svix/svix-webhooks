// this file is @generated
import {
  AttemptStatisticsResponse,
  AttemptStatisticsResponseSerializer,
} from "../models/attemptStatisticsResponse";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface StatsAppAttemptsOptions {
  /** Filter the range to data starting from this date. */
  startDate?: Date | null;
  /** Filter the range to data ending by this date. */
  endDate?: Date | null;
}

export interface StatsEndpointAttemptsOptions {
  /** Filter the range to data starting from this date. */
  startDate?: Date | null;
  /** Filter the range to data ending by this date. */
  endDate?: Date | null;
}

export class Stats {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Returns application-level statistics on message attempts */
  public appAttempts(
    appId: string,
    options?: StatsAppAttemptsOptions
  ): Promise<AttemptStatisticsResponse> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/stats/app/{app_id}/attempt");

    request.setPathParam("app_id", appId);
    request.setQueryParam("startDate", options?.startDate);
    request.setQueryParam("endDate", options?.endDate);

    return request.send(
      this.requestCtx,
      AttemptStatisticsResponseSerializer._fromJsonObject
    );
  }

  /** Returns endpoint-level statistics on message attempts. */
  public endpointAttempts(
    appId: string,
    endpointId: string,
    options?: StatsEndpointAttemptsOptions
  ): Promise<AttemptStatisticsResponse> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/stats/app/{app_id}/ep/{endpoint_id}/attempt"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setQueryParam("startDate", options?.startDate);
    request.setQueryParam("endDate", options?.endDate);

    return request.send(
      this.requestCtx,
      AttemptStatisticsResponseSerializer._fromJsonObject
    );
  }
}
