from enum import Enum


class MessageAttemptExhaustedEventType(str, Enum):
    MESSAGE_ATTEMPT_EXHAUSTED = "message.attempt.exhausted"

    def __str__(self) -> str:
        return str(self.value)
