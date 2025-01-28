# this file is @generated
from enum import Enum


class BackgroundTaskType(str, Enum):
    ENDPOINT_REPLAY = "endpoint.replay"
    ENDPOINT_RECOVER = "endpoint.recover"
    APPLICATION_STATS = "application.stats"
    MESSAGE_BROADCAST = "message.broadcast"
    SDK_GENERATE = "sdk.generate"
    EVENT_TYPE_AGGREGATE = "event-type.aggregate"

    def __str__(self) -> str:
        return str(self.value)
