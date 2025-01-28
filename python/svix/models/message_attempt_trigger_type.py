# this file is @generated
from enum import IntEnum


class MessageAttemptTriggerType(IntEnum):
    SCHEDULED = 0
    MANUAL = 1

    def __str__(self) -> str:
        return str(self.value)
