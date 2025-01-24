from enum import Enum


class SinkInType1Type(str, Enum):
    SQS = "sqs"

    def __str__(self) -> str:
        return str(self.value)
