// this file is @generated

export interface ClickhouseConfig {
  /** The Clickhouse database to connect to */
  database?: string;
  /** Password to access Clickhouse */
  password: string;
  /** The Clickhouse table to write to */
  tableName: string;
  /** The HTTP URL of the ClickHouse server (e.g. `https://my_clickhouse:8443`). */
  url: string;
  /** Username to access Clickhouse */
  username: string;
}

export const ClickhouseConfigSerializer = {
  _fromJsonObject(object: any): ClickhouseConfig {
    return {
      database: object["database"],
      password: object["password"],
      tableName: object["tableName"],
      url: object["url"],
      username: object["username"],
    };
  },

  _toJsonObject(self: ClickhouseConfig): any {
    return {
      database: self.database,
      password: self.password,
      tableName: self.tableName,
      url: self.url,
      username: self.username,
    };
  },
};
