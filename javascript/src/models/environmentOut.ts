// this file is @generated
import { type ConnectorOut, ConnectorOutSerializer } from "./connectorOut";
import { type EventTypeOut, EventTypeOutSerializer } from "./eventTypeOut";

export interface EnvironmentOut {
  version?: number;
  createdAt: Date;
  eventTypes: EventTypeOut[];
  settings: any | null;
  connectors: ConnectorOut[];
}

export const EnvironmentOutSerializer = {
  _fromJsonObject(object: any): EnvironmentOut {
    return {
      version: object["version"],
      createdAt: new Date(object["createdAt"]),
      eventTypes: object["eventTypes"].map((item: EventTypeOut) =>
        EventTypeOutSerializer._fromJsonObject(item)
      ),
      settings: object["settings"],
      connectors: object["connectors"].map((item: ConnectorOut) =>
        ConnectorOutSerializer._fromJsonObject(item)
      ),
    };
  },

  _toJsonObject(self: EnvironmentOut): any {
    return {
      version: self.version,
      createdAt: self.createdAt,
      eventTypes: self.eventTypes.map((item) =>
        EventTypeOutSerializer._toJsonObject(item)
      ),
      settings: self.settings,
      connectors: self.connectors.map((item) =>
        ConnectorOutSerializer._toJsonObject(item)
      ),
    };
  },
};
