# this file is @generated
import typing as t

from .common import BaseModel


class PostgresConfigOut(BaseModel):
    url: str

    table_name: str

    ssl_root_cert: t.Optional[str] = None
