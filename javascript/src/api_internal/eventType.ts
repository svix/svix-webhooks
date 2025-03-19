// this file is @generated
import {
  EventTypeExampleOut,
  EventTypeExampleOutSerializer,
} from "../models/eventTypeExampleOut";
import {
  EventTypeSchemaIn,
  EventTypeSchemaInSerializer,
} from "../models/eventTypeSchemaIn";
import {
  ExportEventTypeOut,
  ExportEventTypeOutSerializer,
} from "../models/exportEventTypeOut";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface EventTypeExportOpenapiOptions {
  idempotencyKey?: string;
}

export interface EventTypeGenerateExampleOptions {
  idempotencyKey?: string;
}

export class EventType {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /**
   * Exports event type definitions based on the OpenAPI schemas associated
   * with each existing event type.
   */
  public exportOpenapi(
    options?: EventTypeExportOpenapiOptions
  ): Promise<ExportEventTypeOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/event-type/export/openapi");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);

    return request.send(this.requestCtx, ExportEventTypeOutSerializer._fromJsonObject);
  }

  /** Generates a fake example from the given JSONSchema. */
  public generateExample(
    eventTypeSchemaIn: EventTypeSchemaIn,
    options?: EventTypeGenerateExampleOptions
  ): Promise<EventTypeExampleOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/event-type/schema/generate-example"
    );

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(EventTypeSchemaInSerializer._toJsonObject(eventTypeSchemaIn));

    return request.send(this.requestCtx, EventTypeExampleOutSerializer._fromJsonObject);
  }
}
