# this file is @generated
import typing as t

from .common import BaseModel


class RedshiftConfigOut(BaseModel):
    access_key_id: str

    region: str

    cluster_identifier: t.Optional[str] = None

    db_user: t.Optional[str] = None

    workgroup_name: t.Optional[str] = None

    db_name: t.Optional[str] = None
    """Database name.

    Only required if not using transformations."""

    schema_name: t.Optional[str] = None
    """Schema name.

    Only used if not using transformations."""

    table_name: t.Optional[str] = None
    """Table name.

    Only required if not using transformations."""
