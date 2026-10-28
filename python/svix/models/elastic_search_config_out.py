# this file is @generated

from .common import BaseModel
from .elastic_search_operation import ElasticSearchOperation


class ElasticSearchConfigOut(BaseModel):
    index_name: str

    url: str

    refresh: bool

    operation: ElasticSearchOperation
