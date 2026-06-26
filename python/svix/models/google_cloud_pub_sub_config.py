# this file is @generated

from .common import BaseModel


class GoogleCloudPubSubConfig(BaseModel):
    credentials: str
    """Google Cloud Credentials JSON Object as a string."""

    project_id: str

    topic_id: str
