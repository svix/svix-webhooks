# this file is @generated
import typing as t

from .common import BaseModel


class ClickhouseConfigIn(BaseModel):
    url: str
    """The HTTP URL of the ClickHouse server (e.g. `https://my_clickhouse:8443`)."""

    username: t.Optional[str] = None
    """Username to access Clickhouse.

    Currently a required field, but marked as optional because we may add different authentication in the future."""

    password: t.Optional[str] = None
    """Password to access Clickhouse.

    Currently a required field, but marked as optional because we may add different authentication in the future."""

    database: t.Optional[str] = None
    """The Clickhouse database to connect to."""

    table_name: str
    """The Clickhouse table to write to."""
