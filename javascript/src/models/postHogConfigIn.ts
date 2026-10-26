// this file is @generated

/** Configuration parameters for defining a PostHog sink. */
export interface PostHogConfigIn {
  /**
   * The PostHog ingestion domain.
   *
   * This is https://us.i.posthog.com for US Cloud, https://eu.i.posthog.com for EU Cloud,
   * and your self-hosted domain for self-hosted instances. Note the `.i.` in the Cloud hosts
   * (`us.posthog.com` and `eu.posthog.com` are different, private endpoints).
   */
  domain: string;
  /**
   * The PostHog project API key.
   *
   * Sent as the `api_key` field in the body of each `/batch/` request.
   */
  projectToken: string;
}

export const PostHogConfigInSerializer = {
  _fromJsonObject(object: any): PostHogConfigIn {
    return {
      domain: object["domain"],
      projectToken: object["projectToken"],
    };
  },

  _toJsonObject(self: PostHogConfigIn): any {
    return {
      domain: self.domain,
      projectToken: self.projectToken,
    };
  },
};
