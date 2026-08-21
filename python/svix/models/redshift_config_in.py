# this file is @generated
import typing as t

from .common import BaseModel


class RedshiftConfigIn(BaseModel):
    """Configuration parameters for defining a Redshift sink.

    For provisioned clusters, set `cluster_identifier` and `db_user`. For Redshift Serverless, set `workgroup_name`."""

    access_key_id: t.Optional[str] = None
    """Access key ID.

    Currently a required field, but marked as optional because we may add different authentication in the future."""

    secret_access_key: t.Optional[str] = None
    """Secret access key.

    Currently a required field, but marked as optional because we may add different authentication in the future."""

    region: t.Optional[str] = None
    """The region of the Redshift DB.

    Currently a required field, but marked as optional because we may infer it from other fields in the future."""

    cluster_identifier: t.Optional[str] = None
    """Required for provisioned clusters."""

    db_user: t.Optional[str] = None
    """Required for provisioned clusters."""

    workgroup_name: t.Optional[str] = None
    """Required for Redshift Serverless."""

    db_name: t.Optional[str] = None
    """Database name.

    Only required if not using transformations."""

    schema_name: t.Optional[str] = None
    """Schema name.

    Only used if not using transformations."""

    table_name: t.Optional[str] = None
    """Table name.

    Only required if not using transformations."""
