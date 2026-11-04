# this file is @generated

from .common import BaseModel


class GenericWebhookResponse(BaseModel):
    content_type: str

    body: str
