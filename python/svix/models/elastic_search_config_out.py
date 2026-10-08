# this file is @generated

from .common import BaseModel


class ElasticSearchConfigOut(BaseModel):
    index_name: str

    url: str

    refresh: bool
