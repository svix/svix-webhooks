# this file is @generated

from .common import BaseModel


class BigQueryConfig(BaseModel):
    """Configuration for a Google Cloud BigQuery sink."""

    credentials: str
    """Google Cloud Credentials JSON Object as a string."""

    dataset_id: str

    project_id: str

    table_id: str
