// this file is @generated

export interface TailscaleConfig {
  /** Shared secret for Tailscale Webhooks */
  secret: string;
  /**
   * Grace period (in seconds) for the timestamp.
   *
   * If not passed, timestamp age will not be checked.
   */
  timestampGraceSeconds?: number | null;
}

export const TailscaleConfigSerializer = {
  _fromJsonObject(object: any): TailscaleConfig {
    return {
      secret: object["secret"],
      timestampGraceSeconds: object["timestampGraceSeconds"],
    };
  },

  _toJsonObject(self: TailscaleConfig): any {
    return {
      secret: self.secret,
      timestampGraceSeconds: self.timestampGraceSeconds,
    };
  },
};
