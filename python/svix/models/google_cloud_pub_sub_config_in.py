# this file is @generated

from .common import BaseModel


class GoogleCloudPubSubConfigIn(BaseModel):
    project_id: str

    topic_id: str

    credentials: str
    """Google Cloud Credentials JSON Object as a string."""
