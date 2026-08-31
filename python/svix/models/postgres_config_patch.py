# this file is @generated
import typing as t

from .common import BaseModel


class PostgresConfigPatch(BaseModel):
    url: t.Optional[str] = None

    password: t.Optional[str] = None

    table_name: t.Optional[str] = None

    ssl_root_cert: t.Optional[str] = None
