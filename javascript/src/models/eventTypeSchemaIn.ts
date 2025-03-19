// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EventTypeSchemaIn {
  schema: any;
}

export const EventTypeSchemaInSerializer = {
  _fromJsonObject(object: any): EventTypeSchemaIn {
    return {
      schema: object["schema"],
    };
  },

  _toJsonObject(self: EventTypeSchemaIn): any {
    return {
      schema: self.schema,
    };
  },
};
