# this file is @generated
from enum import IntEnum


class MessageStatus(IntEnum):
    SUCCESS = 0
    PENDING = 1
    FAIL = 2
    SENDING = 3

    def __str__(self) -> str:
        return str(self.value)
