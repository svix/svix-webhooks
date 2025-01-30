# this file is @generated
import typing as t

from .common import BaseModel


class OperationalWebhookEndpointHeadersOut(BaseModel):
    headers: t.Dict[str, str]

    sensitive: t.List[str]
