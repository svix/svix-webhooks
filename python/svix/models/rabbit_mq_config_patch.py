# this file is @generated
import typing as t

from .common import BaseModel


class RabbitMqConfigPatch(BaseModel):
    routing_key: t.Optional[str] = None

    uri: t.Optional[str] = None
