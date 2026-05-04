# this file is @generated
from enum import Enum


class MessageStatusText(str, Enum):
    SUCCESS = "success"
    PENDING = "pending"
    FAIL = "fail"
    SENDING = "sending"
    CANCELED = "canceled"

    def __str__(self) -> str:
        return str(self.value)
