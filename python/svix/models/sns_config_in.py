# this file is @generated
import typing as t

from .common import BaseModel


class SnsConfigIn(BaseModel):
    """Configuration for a SNS sink."""

    topic_arn: str

    region: t.Optional[str] = None
    """The region of the SNS instance.

    Currently a required field, but marked as optional because we may infer it from other fields in the future."""

    access_key_id: t.Optional[str] = None
    """Access key ID.

    Required (along with `secret_access_key`) if `role_arn` is None"""

    secret_access_key: t.Optional[str] = None
    """Secret access key.

    Required (along with `access_key_id`) if `role_arn` is None"""

    endpoint_url: t.Optional[str] = None

    role_arn: t.Optional[str] = None
    """Role ARN for delegated authentication"""

    external_id: t.Optional[str] = None
    """Shared secret passed as the STS ExternalId.

    Can only be set if `role_arn` is Some"""
