# this file is @generated

from .common import BaseModel


class GoogleCloudStorageConfig(BaseModel):
    """Configuration for a Google Cloud Storage sink.

    Write stream events into the named bucket using the supplied Google Cloud credentials."""

    bucket: str

    credentials: str
    """Google Cloud Credentials JSON Object as a string."""
