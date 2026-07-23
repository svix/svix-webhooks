// this file is @generated

export interface GoogleCloudPubSubConfig {
  projectId: string;
  topicId: string;
  /** Google Cloud Credentials JSON Object as a string. */
  credentials: string;
}

export const GoogleCloudPubSubConfigSerializer = {
  _fromJsonObject(object: any): GoogleCloudPubSubConfig {
    return {
      projectId: object["projectId"],
      topicId: object["topicId"],
      credentials: object["credentials"],
    };
  },

  _toJsonObject(self: GoogleCloudPubSubConfig): any {
    return {
      projectId: self.projectId,
      topicId: self.topicId,
      credentials: self.credentials,
    };
  },
};
