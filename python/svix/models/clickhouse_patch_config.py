# this file is @generated
import typing as t

from .common import BaseModel


class ClickhousePatchConfig(BaseModel):
    url: t.Optional[str] = None

    username: t.Optional[str] = None

    password: t.Optional[str] = None

    database: t.Optional[str] = None

    table_name: t.Optional[str] = None
