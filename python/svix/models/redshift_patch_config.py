# this file is @generated
import typing as t

from .common import BaseModel


class RedshiftPatchConfig(BaseModel):
    access_key_id: t.Optional[str] = None

    secret_access_key: t.Optional[str] = None

    region: t.Optional[str] = None

    db_name: t.Optional[str] = None
    """Database name.

    Only required if not using transformations."""

    schema_name: t.Optional[str] = None
    """Schema name.

    Only used if not using transformations."""

    table_name: t.Optional[str] = None
    """Table name.

    Only required if not using transformations."""
