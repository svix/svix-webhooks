// this file is @generated

export interface GoogleCloudPubSubConfigIn {
  projectId: string;
  topicId: string;
  /** Google Cloud Credentials JSON Object as a string. */
  credentials: string;
}

export const GoogleCloudPubSubConfigInSerializer = {
  _fromJsonObject(object: any): GoogleCloudPubSubConfigIn {
    return {
      projectId: object["projectId"],
      topicId: object["topicId"],
      credentials: object["credentials"],
    };
  },

  _toJsonObject(self: GoogleCloudPubSubConfigIn): any {
    return {
      projectId: self.projectId,
      topicId: self.topicId,
      credentials: self.credentials,
    };
  },
};
