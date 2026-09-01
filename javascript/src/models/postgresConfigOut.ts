// this file is @generated

export interface PostgresConfigOut {
  url: string;
  tableName: string;
  sslRootCert?: string | null;
}

export const PostgresConfigOutSerializer = {
  _fromJsonObject(object: any): PostgresConfigOut {
    return {
      url: object["url"],
      tableName: object["tableName"],
      sslRootCert: object["sslRootCert"],
    };
  },

  _toJsonObject(self: PostgresConfigOut): any {
    return {
      url: self.url,
      tableName: self.tableName,
      sslRootCert: self.sslRootCert,
    };
  },
};
