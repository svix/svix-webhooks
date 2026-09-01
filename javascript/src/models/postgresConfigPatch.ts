// this file is @generated

export interface PostgresConfigPatch {
  url?: string;
  password?: string;
  tableName?: string;
  sslRootCert?: string;
}

export const PostgresConfigPatchSerializer = {
  _fromJsonObject(object: any): PostgresConfigPatch {
    return {
      url: object["url"],
      password: object["password"],
      tableName: object["tableName"],
      sslRootCert: object["sslRootCert"],
    };
  },

  _toJsonObject(self: PostgresConfigPatch): any {
    return {
      url: self.url,
      password: self.password,
      tableName: self.tableName,
      sslRootCert: self.sslRootCert,
    };
  },
};
