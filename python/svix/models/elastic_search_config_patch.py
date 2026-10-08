# this file is @generated
import typing as t

from .common import BaseModel


class ElasticSearchConfigPatch(BaseModel):
    index_name: t.Optional[str] = None

    url: t.Optional[str] = None

    api_key: t.Optional[str] = None

    refresh: t.Optional[bool] = None
