from enum import Enum


class MessageAttemptRecoveredEventType(str, Enum):
    MESSAGE_ATTEMPT_RECOVERED = "message.attempt.recovered"

    def __str__(self) -> str:
        return str(self.value)
