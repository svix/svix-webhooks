# this file is @generated
import typing as t

from .common import BaseModel


class S3ConfigIn(BaseModel):
    bucket: str

    access_key_id: t.Optional[str] = None
    """Access key ID.

    Required (along with `secret_access_key`) if `role_arn` is null"""

    secret_access_key: t.Optional[str] = None
    """Secret access key.

    Required (along with `access_key_id`) if `role_arn` is null"""

    region: t.Optional[str] = None
    """The region of the S3 bucket

    Currently a required field, but marked as optional because we may infer it from other fields in the future."""

    endpoint_url: t.Optional[str] = None

    role_arn: t.Optional[str] = None
    """Role ARN for delegated authentication"""

    external_id: t.Optional[str] = None
    """Shared secret passed as the STS ExternalId.

    Required if `role_arn` is not null."""
