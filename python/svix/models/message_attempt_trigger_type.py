# this file is @generated
from enum import IntEnum


class MessageAttemptTriggerType(IntEnum):
    """The reason an attempt was made:
    - Scheduled = 0
    - Manual = 1"""

    SCHEDULED = 0
    MANUAL = 1

    def __str__(self) -> str:
        return str(self.value)
