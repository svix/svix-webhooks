# this file is @generated

from .common import BaseModel


class RabbitMqConfigIn(BaseModel):
    """Configuration for a RabbitMq sink."""

    uri: str

    routing_key: str
