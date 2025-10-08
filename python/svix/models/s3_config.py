# this file is @generated

from .common import BaseModel


class S3Config(BaseModel):
    access_key_id: str

    bucket: str

    region: str

    secret_access_key: str
