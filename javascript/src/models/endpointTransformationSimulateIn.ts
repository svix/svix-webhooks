// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EndpointTransformationSimulateIn {
  channels?: string[] | null;
  code: string;
  /** The event type's name */
  eventType: string;
  payload: any;
}

export const EndpointTransformationSimulateInSerializer = {
  _fromJsonObject(object: any): EndpointTransformationSimulateIn {
    return {
      channels: object["channels"],
      code: object["code"],
      eventType: object["eventType"],
      payload: object["payload"],
    };
  },

  _toJsonObject(self: EndpointTransformationSimulateIn): any {
    return {
      channels: self.channels,
      code: self.code,
      eventType: self.eventType,
      payload: self.payload,
    };
  },
};
