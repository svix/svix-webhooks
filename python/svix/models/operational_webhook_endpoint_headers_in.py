# this file is @generated
import typing as t

from .common import BaseModel


class OperationalWebhookEndpointHeadersIn(BaseModel):
    headers: t.Dict[str, str]
