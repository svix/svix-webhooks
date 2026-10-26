# this file is @generated
import typing as t

from .common import BaseModel
from .elastic_search_operation import ElasticSearchOperation


class ElasticSearchConfigPatch(BaseModel):
    index_name: t.Optional[str] = None

    url: t.Optional[str] = None

    api_key: t.Optional[str] = None

    refresh: t.Optional[bool] = None

    operation: t.Optional[ElasticSearchOperation] = None
