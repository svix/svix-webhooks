# this file is @generated
import typing as t

from .common import BaseModel


class SnowflakeConfig(BaseModel):
    """Configuration parameters for defining a Snowflake sink."""

    private_key: str
    """PEM-encoded private key used for signing token-based requests to the Snowflake API.

    Beginning/end delimiters are not required."""

    account_identifier: str
    """Snowflake account identifier, which includes both the organization and account IDs separated by a hyphen."""

    user_id: str
    """The Snowflake user id."""

    db_name: t.Optional[str] = None
    """Database name.

    Only required if not using transformations."""

    schema_name: t.Optional[str] = None
    """Schema name.

    Only required if not using transformations."""

    table_name: t.Optional[str] = None
    """Table name.

    Only required if not using transformations."""
