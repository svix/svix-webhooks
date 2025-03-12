// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { ConnectorOut, ConnectorOutSerializer } from "./connectorOut";
import { EventTypeOut, EventTypeOutSerializer } from "./eventTypeOut";

export interface EnvironmentOut {
  createdAt: Date;
  eventTypes: EventTypeOut[];
  settings: any | null;
  transformationTemplates: ConnectorOut[];
  version?: number;
}

export const EnvironmentOutSerializer = {
  _fromJsonObject(object: any): EnvironmentOut {
    return {
      createdAt: new Date(object["createdAt"]),
      eventTypes: object["eventTypes"].map((item: EventTypeOut) =>
        EventTypeOutSerializer._fromJsonObject(item)
      ),
      settings: object["settings"],
      transformationTemplates: object["transformationTemplates"].map(
        (item: ConnectorOut) => ConnectorOutSerializer._fromJsonObject(item)
      ),
      version: object["version"],
    };
  },

  _toJsonObject(self: EnvironmentOut): any {
    return {
      createdAt: self.createdAt,
      eventTypes: self.eventTypes.map((item) =>
        EventTypeOutSerializer._toJsonObject(item)
      ),
      settings: self.settings,
      transformationTemplates: self.transformationTemplates.map((item) =>
        ConnectorOutSerializer._toJsonObject(item)
      ),
      version: self.version,
    };
  },
};
