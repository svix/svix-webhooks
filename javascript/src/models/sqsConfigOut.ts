// this file is @generated

export interface SqsConfigOut {
  queueUrl: string;
  region: string;
  accessKeyId?: string | null;
  roleArn?: string | null;
  externalId?: string | null;
  endpointUrl?: string | null;
}

export const SqsConfigOutSerializer = {
  _fromJsonObject(object: any): SqsConfigOut {
    return {
      queueUrl: object["queueUrl"],
      region: object["region"],
      accessKeyId: object["accessKeyId"],
      roleArn: object["roleArn"],
      externalId: object["externalId"],
      endpointUrl: object["endpointUrl"],
    };
  },

  _toJsonObject(self: SqsConfigOut): any {
    return {
      queueUrl: self.queueUrl,
      region: self.region,
      accessKeyId: self.accessKeyId,
      roleArn: self.roleArn,
      externalId: self.externalId,
      endpointUrl: self.endpointUrl,
    };
  },
};
