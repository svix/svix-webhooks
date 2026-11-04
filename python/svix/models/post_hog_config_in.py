# this file is @generated

from .common import BaseModel


class PostHogConfigIn(BaseModel):
    """Configuration parameters for defining a PostHog sink."""

    domain: str
    """The PostHog ingestion domain.

    This is https://us.i.posthog.com for US Cloud, https://eu.i.posthog.com for EU Cloud,
    and your self-hosted domain for self-hosted instances. Note the `.i.` in the Cloud hosts
    (`us.posthog.com` and `eu.posthog.com` are different, private endpoints)."""

    project_token: str
    """The PostHog project API key.

    Sent as the `api_key` field in the body of each `/batch/` request."""
