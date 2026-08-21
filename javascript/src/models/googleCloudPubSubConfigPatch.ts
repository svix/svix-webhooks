// this file is @generated

export interface GoogleCloudPubSubConfigPatch {
  projectId?: string;
  topicId?: string;
  credentials?: string;
}

export const GoogleCloudPubSubConfigPatchSerializer = {
  _fromJsonObject(object: any): GoogleCloudPubSubConfigPatch {
    return {
      projectId: object["projectId"],
      topicId: object["topicId"],
      credentials: object["credentials"],
    };
  },

  _toJsonObject(self: GoogleCloudPubSubConfigPatch): any {
    return {
      projectId: self.projectId,
      topicId: self.topicId,
      credentials: self.credentials,
    };
  },
};
