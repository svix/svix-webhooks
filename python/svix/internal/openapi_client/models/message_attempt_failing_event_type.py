from enum import Enum


class MessageAttemptFailingEventType(str, Enum):
    MESSAGE_ATTEMPT_FAILING = "message.attempt.failing"

    def __str__(self) -> str:
        return str(self.value)
