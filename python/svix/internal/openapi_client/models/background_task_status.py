from enum import Enum


class BackgroundTaskStatus(str, Enum):
    FAILED = "failed"
    FINISHED = "finished"
    RUNNING = "running"

    def __str__(self) -> str:
        return str(self.value)
