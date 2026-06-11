// this file is @generated

export enum StartingPosition {
  Earliest = "earliest",
  Latest = "latest",
}

export const StartingPositionSerializer = {
  _fromJsonObject(object: any): StartingPosition {
    return object;
  },

  _toJsonObject(self: StartingPosition): any {
    return self;
  },
};
