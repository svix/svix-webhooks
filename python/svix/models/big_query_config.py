# this file is @generated

from .common import BaseModel


class BigQueryConfig(BaseModel):
    """Configuration for a Google Cloud BigQuery sink."""

    project_id: str

    dataset_id: str

    table_id: str

    credentials: str
    """Google Cloud Credentials JSON Object as a string."""
