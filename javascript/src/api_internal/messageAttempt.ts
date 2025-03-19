// this file is @generated
import { CountOut, CountOutSerializer } from "../models/countOut";
import {
  MessageAttemptHeadersOut,
  MessageAttemptHeadersOutSerializer,
} from "../models/messageAttemptHeadersOut";
import { MessageStatus } from "../models/messageStatus";
import { StatusCodeClass } from "../models/statusCodeClass";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface MessageAttemptCountByEndpointOptions {
  /** Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3) */
  status?: MessageStatus;
  /** Filter response based on the HTTP status code */
  statusCodeClass?: StatusCodeClass;
  /** Filter response based on the channel */
  channel?: string;
  /** Filter response based on the tag */
  tag?: string;
  /** Only include items created before a certain date */
  before?: Date | null;
  /** Only include items created after a certain date */
  after?: Date | null;
  /** Filter response based on the event type */
  eventTypes?: string[];
}

export class MessageAttempt {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Like `v1.message-attempt.list-by-endpoint` but returning a count only. */
  public countByEndpoint(
    appId: string,
    endpointId: string,
    options?: MessageAttemptCountByEndpointOptions
  ): Promise<CountOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/attempt/endpoint/{endpoint_id}/count"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setQueryParam("status", options?.status);
    request.setQueryParam("status_code_class", options?.statusCodeClass);
    request.setQueryParam("channel", options?.channel);
    request.setQueryParam("tag", options?.tag);
    request.setQueryParam("before", options?.before);
    request.setQueryParam("after", options?.after);
    request.setQueryParam("event_types", options?.eventTypes);

    return request.send(this.requestCtx, CountOutSerializer._fromJsonObject);
  }

  /** Calculate and return headers used on a given message attempt */
  public getHeaders(
    appId: string,
    msgId: string,
    attemptId: string
  ): Promise<MessageAttemptHeadersOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}/headers"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("msg_id", msgId);
    request.setPathParam("attempt_id", attemptId);

    return request.send(
      this.requestCtx,
      MessageAttemptHeadersOutSerializer._fromJsonObject
    );
  }
}
