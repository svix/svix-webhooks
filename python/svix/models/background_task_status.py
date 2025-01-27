# this file is @generated
from enum import Enum


class BackgroundTaskStatus(str, Enum):
    RUNNING = "running"
    FINISHED = "finished"
    FAILED = "failed"

    def __str__(self) -> str:
        return str(self.value)
