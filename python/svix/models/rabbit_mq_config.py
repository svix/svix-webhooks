# this file is @generated

from .common import BaseModel


class RabbitMqConfig(BaseModel):
    """Configuration for a RabbitMq sink."""

    uri: str

    routing_key: str
