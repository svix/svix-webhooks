# this file is @generated

from .common import BaseModel


class RabbitMqConfigOut(BaseModel):
    uri: str

    routing_key: str
