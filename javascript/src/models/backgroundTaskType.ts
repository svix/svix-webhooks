// this file is @generated

export enum BackgroundTaskType {
  EndpointReplay = "endpoint.replay",
  EndpointRecover = "endpoint.recover",
  ApplicationStats = "application.stats",
  MessageBroadcast = "message.broadcast",
  SdkGenerate = "sdk.generate",
  EventTypeAggregate = "event-type.aggregate",
  ApplicationPurgeContent = "application.purge_content",
  EndpointBulkReplay = "endpoint.bulk_replay",
}

export const BackgroundTaskTypeSerializer = {
  _fromJsonObject(object: any): BackgroundTaskType {
    return object;
  },

  _toJsonObject(self: BackgroundTaskType): any {
    return self;
  },
};
