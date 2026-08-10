# this file is @generated

from .common import BaseModel


class ClickhouseConfigOut(BaseModel):
    url: str

    username: str

    database: str

    table_name: str
