# this file is @generated
import typing as t

from .common import BaseModel


class SnowflakeConfigOut(BaseModel):
    account_identifier: str

    user_id: str

    db_name: t.Optional[str] = None
    """Database name.

    Only required if not using transformations."""

    schema_name: t.Optional[str] = None
    """Schema name.

    Only required if not using transformations."""

    table_name: t.Optional[str] = None
    """Table name.

    Only required if not using transformations."""
