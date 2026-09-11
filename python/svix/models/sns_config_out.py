# this file is @generated
import typing as t

from .common import BaseModel


class SnsConfigOut(BaseModel):
    topic_arn: str

    region: str

    access_key_id: t.Optional[str] = None

    role_arn: t.Optional[str] = None

    external_id: t.Optional[str] = None
