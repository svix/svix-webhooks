// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export enum ConnectorKind {
  Custom = "Custom",
  CloseCrm = "CloseCRM",
  CustomerIo = "CustomerIO",
  Discord = "Discord",
  Hubspot = "Hubspot",
  Inngest = "Inngest",
  Loops = "Loops",
  Resend = "Resend",
  Salesforce = "Salesforce",
  Segment = "Segment",
  Sendgrid = "Sendgrid",
  Slack = "Slack",
  Teams = "Teams",
  TriggerDev = "TriggerDev",
  Windmill = "Windmill",
  Zapier = "Zapier",
}

export const ConnectorKindSerializer = {
  _fromJsonObject(object: any): ConnectorKind {
    return object;
  },

  _toJsonObject(self: ConnectorKind): any {
    return self;
  },
};
