// this file is @generated
import { type Status, StatusSerializer } from "./status";

export interface AutoConfigSubscriptionOut {
  createdAt: Date;
  tokenCensored: string;
  /** The AutoConfigSubscription's ID. */
  id: string;
  /** The Endpoint's ID. */
  endpId?: string | null;
  /** The StreamSink's ID. */
  destId?: string | null;
  status: Status;
}

export const AutoConfigSubscriptionOutSerializer = {
  _fromJsonObject(object: any): AutoConfigSubscriptionOut {
    return {
      createdAt: new Date(object["createdAt"]),
      tokenCensored: object["tokenCensored"],
      id: object["id"],
      endpId: object["endpId"],
      destId: object["destId"],
      status: StatusSerializer._fromJsonObject(object["status"]),
    };
  },

  _toJsonObject(self: AutoConfigSubscriptionOut): any {
    return {
      createdAt: self.createdAt,
      tokenCensored: self.tokenCensored,
      id: self.id,
      endpId: self.endpId,
      destId: self.destId,
      status: StatusSerializer._toJsonObject(self.status),
    };
  },
};
