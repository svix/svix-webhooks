// this file is @generated
import { type ConnectorOut, ConnectorOutSerializer } from "./connectorOut";
import { type EventTypeOut, EventTypeOutSerializer } from "./eventTypeOut";

export interface EnvironmentOut {
  connectors: ConnectorOut[];
  createdAt: Date;
  eventTypes: EventTypeOut[];
  settings: any | null;
  version?: number;
}

export const EnvironmentOutSerializer = {
  _fromJsonObject(object: any): EnvironmentOut {
    return {
      connectors: object["connectors"].map((item: ConnectorOut) =>
        ConnectorOutSerializer._fromJsonObject(item)
      ),
      createdAt: new Date(object["createdAt"]),
      eventTypes: object["eventTypes"].map((item: EventTypeOut) =>
        EventTypeOutSerializer._fromJsonObject(item)
      ),
      settings: object["settings"],
      version: object["version"],
    };
  },

  _toJsonObject(self: EnvironmentOut): any {
    return {
      connectors: self.connectors.map((item) =>
        ConnectorOutSerializer._toJsonObject(item)
      ),
      createdAt: self.createdAt,
      eventTypes: self.eventTypes.map((item) =>
        EventTypeOutSerializer._toJsonObject(item)
      ),
      settings: self.settings,
      version: self.version,
    };
  },
};
