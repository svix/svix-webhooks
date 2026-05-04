# this file is @generated
from enum import IntEnum


class MessageStatus(IntEnum):
    """The sending status of the message:

    - Success = 0
    - Pending = 1
    - Fail = 2
    - Sending = 3
    - Canceled = 4"""

    SUCCESS = 0
    PENDING = 1
    FAIL = 2
    SENDING = 3
    CANCELED = 4

    def __str__(self) -> str:
        return str(self.value)
