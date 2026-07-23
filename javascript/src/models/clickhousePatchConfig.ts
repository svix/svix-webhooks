// this file is @generated

export interface ClickhousePatchConfig {
  url?: string;
  username?: string;
  password?: string;
  database?: string;
  tableName?: string;
}

export const ClickhousePatchConfigSerializer = {
  _fromJsonObject(object: any): ClickhousePatchConfig {
    return {
      url: object["url"],
      username: object["username"],
      password: object["password"],
      database: object["database"],
      tableName: object["tableName"],
    };
  },

  _toJsonObject(self: ClickhousePatchConfig): any {
    return {
      url: self.url,
      username: self.username,
      password: self.password,
      database: self.database,
      tableName: self.tableName,
    };
  },
};
