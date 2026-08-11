// this file is @generated

export interface ClickhouseConfigIn {
  /** The HTTP URL of the ClickHouse server (e.g. `https://my_clickhouse:8443`). */
  url: string;
  /** Username to access Clickhouse */
  username: string;
  /** Password to access Clickhouse */
  password: string;
  /** The Clickhouse database to connect to */
  database?: string;
  /** The Clickhouse table to write to */
  tableName: string;
}

export const ClickhouseConfigInSerializer = {
  _fromJsonObject(object: any): ClickhouseConfigIn {
    return {
      url: object["url"],
      username: object["username"],
      password: object["password"],
      database: object["database"],
      tableName: object["tableName"],
    };
  },

  _toJsonObject(self: ClickhouseConfigIn): any {
    return {
      url: self.url,
      username: self.username,
      password: self.password,
      database: self.database,
      tableName: self.tableName,
    };
  },
};
