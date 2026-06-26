// this file is @generated

export interface GoogleCloudPubSubPatchConfig {
  credentials?: string;
  projectId?: string;
  topicId?: string;
}

export const GoogleCloudPubSubPatchConfigSerializer = {
  _fromJsonObject(object: any): GoogleCloudPubSubPatchConfig {
    return {
      credentials: object["credentials"],
      projectId: object["projectId"],
      topicId: object["topicId"],
    };
  },

  _toJsonObject(self: GoogleCloudPubSubPatchConfig): any {
    return {
      credentials: self.credentials,
      projectId: self.projectId,
      topicId: self.topicId,
    };
  },
};
