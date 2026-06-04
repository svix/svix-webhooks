// this file is @generated

export interface GoogleCloudPubSubConfig {
  /** Google Cloud Credentials JSON Object as a string. */
  credentials: string;
  projectId: string;
  topicId: string;
}

export const GoogleCloudPubSubConfigSerializer = {
  _fromJsonObject(object: any): GoogleCloudPubSubConfig {
    return {
      credentials: object["credentials"],
      projectId: object["projectId"],
      topicId: object["topicId"],
    };
  },

  _toJsonObject(self: GoogleCloudPubSubConfig): any {
    return {
      credentials: self.credentials,
      projectId: self.projectId,
      topicId: self.topicId,
    };
  },
};
