# this file is @generated

from .common import BaseModel


class MetaConfig(BaseModel):
    secret: str

    verify_token: str
