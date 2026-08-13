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

    Currently a required field, but marked as optional because we may add different authentication in the future."""

    secret_access_key: t.Optional[str] = None
    """Secret access key.

    Currently a required field, but marked as optional because we may add different authentication in the future."""

    endpoint_url: t.Optional[str] = None
