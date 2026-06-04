# this file is @generated
import typing as t

from .common import BaseModel


class ClickhouseConfig(BaseModel):
    database: t.Optional[str] = None
    """The Clickhouse database to connect to"""

    password: str
    """Password to access Clickhouse"""

    table_name: str
    """The Clickhouse table to write to"""

    url: str
    """The HTTP URL of the ClickHouse server (e.g. `https://my_clickhouse:8443`)."""

    username: str
    """Username to access Clickhouse"""
