// this file is @generated

export interface SqsConfigOut {
  queueUrl: string;
  region: string;
  accessKeyId: string;
  endpointUrl?: string | null;
}

export const SqsConfigOutSerializer = {
  _fromJsonObject(object: any): SqsConfigOut {
    return {
      queueUrl: object["queueUrl"],
      region: object["region"],
      accessKeyId: object["accessKeyId"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: SqsConfigOut): any {
    return {
      queueUrl: self.queueUrl,
      region: self.region,
      accessKeyId: self.accessKeyId,
      endpointUrl: self.endpointUrl,
    };
  },
};
