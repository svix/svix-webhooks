// this file is @generated

export interface ClickhouseConfigOut {
  url: string;
  username: string;
  database: string;
  tableName: string;
}

export const ClickhouseConfigOutSerializer = {
  _fromJsonObject(object: any): ClickhouseConfigOut {
    return {
      url: object["url"],
      username: object["username"],
      database: object["database"],
      tableName: object["tableName"],
    };
  },

  _toJsonObject(self: ClickhouseConfigOut): any {
    return {
      url: self.url,
      username: self.username,
      database: self.database,
      tableName: self.tableName,
    };
  },
};
