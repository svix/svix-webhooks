// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  EndpointDisabledTrigger,
  EndpointDisabledTriggerSerializer,
} from "./endpointDisabledTrigger";

/** Sent when an endpoint has been automatically disabled after continuous failures, or manually via an API call. */
export interface EndpointDisabledEventData {
  /** The Application's ID. */
  appId: string;
  /** The Application's UID. */
  appUid?: string | null;
  /** The Endpoint's ID. */
  endpointId: string;
  /** The Endpoint's UID. */
  endpointUid?: string | null;
  failSince?: Date | null;
  trigger?: EndpointDisabledTrigger;
}

export const EndpointDisabledEventDataSerializer = {
  _fromJsonObject(object: any): EndpointDisabledEventData {
    return {
      appId: object["appId"],
      appUid: object["appUid"],
      endpointId: object["endpointId"],
      endpointUid: object["endpointUid"],
      failSince: object["failSince"] ? new Date(object["failSince"]) : null,
      trigger: object["trigger"]
        ? EndpointDisabledTriggerSerializer._fromJsonObject(object["trigger"])
        : undefined,
    };
  },

  _toJsonObject(self: EndpointDisabledEventData): any {
    return {
      appId: self.appId,
      appUid: self.appUid,
      endpointId: self.endpointId,
      endpointUid: self.endpointUid,
      failSince: self.failSince,
      trigger: self.trigger
        ? EndpointDisabledTriggerSerializer._toJsonObject(self.trigger)
        : undefined,
    };
  },
};
