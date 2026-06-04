# this file is @generated
import typing as t

from .common import BaseModel


class BigQueryPatchConfig(BaseModel):
    credentials: t.Optional[str] = None

    dataset_id: t.Optional[str] = None

    project_id: t.Optional[str] = None

    table_id: t.Optional[str] = None
