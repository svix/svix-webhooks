// this file is @generated

export interface GoogleCloudPubSubConfigOut {
  projectId: string;
  topicId: string;
}

export const GoogleCloudPubSubConfigOutSerializer = {
  _fromJsonObject(object: any): GoogleCloudPubSubConfigOut {
    return {
      projectId: object["projectId"],
      topicId: object["topicId"],
    };
  },

  _toJsonObject(self: GoogleCloudPubSubConfigOut): any {
    return {
      projectId: self.projectId,
      topicId: self.topicId,
    };
  },
};
