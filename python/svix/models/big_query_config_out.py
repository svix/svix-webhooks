# this file is @generated

from .common import BaseModel


class BigQueryConfigOut(BaseModel):
    project_id: str

    dataset_id: str

    table_id: str
