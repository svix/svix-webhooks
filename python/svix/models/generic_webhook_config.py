# this file is @generated
import typing as t

from .common import BaseModel
from .generic_webhook_response import GenericWebhookResponse


class GenericWebhookConfig(BaseModel):
    response: t.Optional[GenericWebhookResponse] = None
