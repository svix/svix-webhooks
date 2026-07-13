# this file is @generated
import typing as t

from .common import BaseModel


class SnowflakePatchConfig(BaseModel):
    private_key: t.Optional[str] = None

    account_identifier: t.Optional[str] = None

    user_id: t.Optional[str] = None

    db_name: t.Optional[str] = None
    """Database name.

    Only required if not using transformations."""

    schema_name: t.Optional[str] = None
    """Schema name.

    Only required if not using transformations."""

    table_name: t.Optional[str] = None
    """Table name.

    Only required if not using transformations."""
