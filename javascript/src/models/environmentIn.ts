// this file is @generated
import { type ConnectorIn, ConnectorInSerializer } from "./connectorIn";
import { type EventTypeIn, EventTypeInSerializer } from "./eventTypeIn";

export interface EnvironmentIn {
  eventTypes?: EventTypeIn[] | null;
  settings?: any | null;
  connectors?: ConnectorIn[] | null;
}

export const EnvironmentInSerializer = {
  _fromJsonObject(object: any): EnvironmentIn {
    return {
      eventTypes: object["eventTypes"]?.map((item: EventTypeIn) =>
        EventTypeInSerializer._fromJsonObject(item)
      ),
      settings: object["settings"],
      connectors: object["connectors"]?.map((item: ConnectorIn) =>
        ConnectorInSerializer._fromJsonObject(item)
      ),
    };
  },

  _toJsonObject(self: EnvironmentIn): any {
    return {
      eventTypes: self.eventTypes?.map((item) =>
        EventTypeInSerializer._toJsonObject(item)
      ),
      settings: self.settings,
      connectors: self.connectors?.map((item) =>
        ConnectorInSerializer._toJsonObject(item)
      ),
    };
  },
};
