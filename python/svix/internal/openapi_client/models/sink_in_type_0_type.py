from enum import Enum


class SinkInType0Type(str, Enum):
    RABBITMQ = "rabbitMQ"

    def __str__(self) -> str:
        return str(self.value)
