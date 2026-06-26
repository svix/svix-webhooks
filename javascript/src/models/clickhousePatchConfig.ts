// this file is @generated

export interface ClickhousePatchConfig {
  database?: string;
  password?: string;
  tableName?: string;
  url?: string;
  username?: string;
}

export const ClickhousePatchConfigSerializer = {
  _fromJsonObject(object: any): ClickhousePatchConfig {
    return {
      database: object["database"],
      password: object["password"],
      tableName: object["tableName"],
      url: object["url"],
      username: object["username"],
    };
  },

  _toJsonObject(self: ClickhousePatchConfig): any {
    return {
      database: self.database,
      password: self.password,
      tableName: self.tableName,
      url: self.url,
      username: self.username,
    };
  },
};
