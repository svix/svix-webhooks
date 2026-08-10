// this file is @generated

export interface ClickhouseConfigPatch {
  url?: string;
  username?: string;
  password?: string;
  database?: string;
  tableName?: string;
}

export const ClickhouseConfigPatchSerializer = {
  _fromJsonObject(object: any): ClickhouseConfigPatch {
    return {
      url: object["url"],
      username: object["username"],
      password: object["password"],
      database: object["database"],
      tableName: object["tableName"],
    };
  },

  _toJsonObject(self: ClickhouseConfigPatch): any {
    return {
      url: self.url,
      username: self.username,
      password: self.password,
      database: self.database,
      tableName: self.tableName,
    };
  },
};
