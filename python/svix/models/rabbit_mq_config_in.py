# this file is @generated
import typing as t

from .common import BaseModel


class RabbitMqConfigIn(BaseModel):
    """Configuration for a RabbitMq sink."""

    uri: str
    """URI to connect to

    Note that the VHost must be percent-escaped, so a default URI would look
    like `amqp://user:pass@host/%2F`"""

    routing_key: str
    """Routing key for message dispatch"""

    mandatory: t.Optional[bool] = None
    """If true, then dispatches will fail if there is no attached queue; if false, they are
    silently dropped (this was previously the default)"""
