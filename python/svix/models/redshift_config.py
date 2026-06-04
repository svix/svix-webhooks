# this file is @generated
import typing as t

from .common import BaseModel


class RedshiftConfig(BaseModel):
    """Configuration parameters for defining a Redshift sink.

    For provisioned clusters, set `cluster_identifier` and `db_user`. For Redshift Serverless, set `workgroup_name`."""

    access_key_id: str

    cluster_identifier: t.Optional[str] = None
    """Required for provisioned clusters."""

    db_name: t.Optional[str] = None
    """Database name.

    Only required if not using transformations."""

    db_user: t.Optional[str] = None
    """Required for provisioned clusters."""

    region: str

    schema_name: t.Optional[str] = None
    """Schema name.

    Only used if not using transformations."""

    secret_access_key: str

    table_name: t.Optional[str] = None
    """Table name.

    Only required if not using transformations."""

    workgroup_name: t.Optional[str] = None
    """Required for Redshift Serverless."""
