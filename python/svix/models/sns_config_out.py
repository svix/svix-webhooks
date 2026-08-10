# this file is @generated

from .common import BaseModel


class SnsConfigOut(BaseModel):
    topic_arn: str

    region: str

    access_key_id: str
