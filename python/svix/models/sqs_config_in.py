# this file is @generated
import typing as t

from .common import BaseModel


class SqsConfigIn(BaseModel):
    """Configuration for an SQS sink."""

    queue_url: str

    region: t.Optional[str] = None
    """The region of the SQS queue.

    Currently a required field, but marked as optional because we may infer it from other fields in the future."""

    access_key_id: t.Optional[str] = None
    """Access key ID.

    Currently a required field, but marked as optional because we may add different authentication in the future."""

    secret_access_key: t.Optional[str] = None
    """Secret access key.

    Currently a required field, but marked as optional because we may add different authentication in the future."""

    endpoint_url: t.Optional[str] = None
