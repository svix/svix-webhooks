// this file is @generated
import {
  type PollerV2MessageOut,
  PollerV2MessageOutSerializer,
} from "./pollerV2MessageOut";

export interface PollerV2PollOut {
  data: PollerV2MessageOut[];
  done: boolean;
}

export const PollerV2PollOutSerializer = {
  _fromJsonObject(object: any): PollerV2PollOut {
    return {
      data: object["data"].map((item: PollerV2MessageOut) =>
        PollerV2MessageOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
    };
  },

  _toJsonObject(self: PollerV2PollOut): any {
    return {
      data: self.data.map((item) => PollerV2MessageOutSerializer._toJsonObject(item)),
      done: self.done,
    };
  },
};
