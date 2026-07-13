// this file is @generated

export interface GoogleCloudPubSubPatchConfig {
  projectId?: string;
  topicId?: string;
  credentials?: string;
}

export const GoogleCloudPubSubPatchConfigSerializer = {
  _fromJsonObject(object: any): GoogleCloudPubSubPatchConfig {
    return {
      projectId: object["projectId"],
      topicId: object["topicId"],
      credentials: object["credentials"],
    };
  },

  _toJsonObject(self: GoogleCloudPubSubPatchConfig): any {
    return {
      projectId: self.projectId,
      topicId: self.topicId,
      credentials: self.credentials,
    };
  },
};
