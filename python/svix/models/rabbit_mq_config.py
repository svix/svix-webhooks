# this file is @generated

from .common import BaseModel


class RabbitMqConfig(BaseModel):
    """Configuration for a RabbitMq sink."""

    routing_key: str

    uri: str
