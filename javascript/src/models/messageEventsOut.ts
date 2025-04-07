// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { MessageOut, MessageOutSerializer } from "./messageOut";

export interface MessageEventsOut {
  data: MessageOut[];
  done: boolean;
  iterator: string;
}

export const MessageEventsOutSerializer = {
  _fromJsonObject(object: any): MessageEventsOut {
    return {
      data: object["data"]?.map((item: MessageOut) =>
        MessageOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
    };
  },

  _toJsonObject(self: MessageEventsOut): any {
    return {
      data: self.data.map((item) => MessageOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
    };
  },
};
