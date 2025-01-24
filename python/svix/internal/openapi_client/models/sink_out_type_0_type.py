from enum import Enum


class SinkOutType0Type(str, Enum):
    RABBITMQ = "rabbitMQ"

    def __str__(self) -> str:
        return str(self.value)
