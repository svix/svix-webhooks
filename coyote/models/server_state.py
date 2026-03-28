# this file is @generated
from enum import Enum


class ServerState(str, Enum):
    LEADER = "Leader"
    FOLLOWER = "Follower"
    LEARNER = "Learner"
    CANDIDATE = "Candidate"
    SHUTDOWN = "Shutdown"
    UNKNOWN = "Unknown"

    def __str__(self) -> str:
        return str(self.value)
