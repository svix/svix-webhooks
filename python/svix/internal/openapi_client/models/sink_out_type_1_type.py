from enum import Enum


class SinkOutType1Type(str, Enum):
    SQS = "sqs"

    def __str__(self) -> str:
        return str(self.value)
