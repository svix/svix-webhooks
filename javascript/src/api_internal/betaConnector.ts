// this file is @generated
import { ConnectorIn, ConnectorInSerializer } from "../models/connectorIn";
import { ConnectorOut, ConnectorOutSerializer } from "../models/connectorOut";
import { ConnectorPatch, ConnectorPatchSerializer } from "../models/connectorPatch";
import { GenerateIn, GenerateInSerializer } from "../models/generateIn";
import { GenerateOut, GenerateOutSerializer } from "../models/generateOut";
import {
  ListResponseConnectorOut,
  ListResponseConnectorOutSerializer,
} from "../models/listResponseConnectorOut";
import { Ordering } from "../models/ordering";
import { TemplateUpdate, TemplateUpdateSerializer } from "../models/templateUpdate";
import {
  TransformationSimulateIn,
  TransformationSimulateInSerializer,
} from "../models/transformationSimulateIn";
import {
  TransformationSimulateOut,
  TransformationSimulateOutSerializer,
} from "../models/transformationSimulateOut";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";
import { BetaConnectorOauth } from "./betaConnectorOauth";

export interface BetaConnectorListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface BetaConnectorCreateOptions {
  idempotencyKey?: string;
}

export interface BetaConnectorGenerateOptions {
  idempotencyKey?: string;
}

export interface BetaConnectorSimulateOptions {
  idempotencyKey?: string;
}

export class BetaConnector {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  public get oauth() {
    return new BetaConnectorOauth(this.requestCtx);
  }

  /** List all transformation templates for an application. */
  public list(options?: BetaConnectorListOptions): Promise<ListResponseConnectorOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/transformation-template");

    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("order", options?.order);

    return request.send(
      this.requestCtx,
      ListResponseConnectorOutSerializer._fromJsonObject
    );
  }

  /** Create a new connector. */
  public create(
    connectorIn: ConnectorIn,
    options?: BetaConnectorCreateOptions
  ): Promise<ConnectorOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/transformation-template");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(ConnectorInSerializer._toJsonObject(connectorIn));

    return request.send(this.requestCtx, ConnectorOutSerializer._fromJsonObject);
  }

  /** Use OpenAI's Completion API to generate code for a connector. */
  public generate(
    generateIn: GenerateIn,
    options?: BetaConnectorGenerateOptions
  ): Promise<GenerateOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/transformation-template/generate"
    );

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(GenerateInSerializer._toJsonObject(generateIn));

    return request.send(this.requestCtx, GenerateOutSerializer._fromJsonObject);
  }

  /** Simulate running the transformation on the payload and code. */
  public simulate(
    transformationSimulateIn: TransformationSimulateIn,
    options?: BetaConnectorSimulateOptions
  ): Promise<TransformationSimulateOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/transformation-template/simulate"
    );

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(
      TransformationSimulateInSerializer._toJsonObject(transformationSimulateIn)
    );

    return request.send(
      this.requestCtx,
      TransformationSimulateOutSerializer._fromJsonObject
    );
  }

  /** Get a connector. */
  public get(transformationTemplateId: string): Promise<ConnectorOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/transformation-template/{transformation_template_id}"
    );

    request.setPathParam("transformation_template_id", transformationTemplateId);

    return request.send(this.requestCtx, ConnectorOutSerializer._fromJsonObject);
  }

  /** Update a connector. */
  public update(
    transformationTemplateId: string,
    templateUpdate: TemplateUpdate
  ): Promise<ConnectorOut> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/transformation-template/{transformation_template_id}"
    );

    request.setPathParam("transformation_template_id", transformationTemplateId);
    request.setBody(TemplateUpdateSerializer._toJsonObject(templateUpdate));

    return request.send(this.requestCtx, ConnectorOutSerializer._fromJsonObject);
  }

  /** Delete a connector. */
  public delete(transformationTemplateId: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/transformation-template/{transformation_template_id}"
    );

    request.setPathParam("transformation_template_id", transformationTemplateId);

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Partially update a connector. */
  public patch(
    transformationTemplateId: string,
    connectorPatch: ConnectorPatch
  ): Promise<ConnectorOut> {
    const request = new SvixRequest(
      HttpMethod.PATCH,
      "/api/v1/transformation-template/{transformation_template_id}"
    );

    request.setPathParam("transformation_template_id", transformationTemplateId);
    request.setBody(ConnectorPatchSerializer._toJsonObject(connectorPatch));

    return request.send(this.requestCtx, ConnectorOutSerializer._fromJsonObject);
  }
}
