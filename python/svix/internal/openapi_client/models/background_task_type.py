from enum import Enum


class BackgroundTaskType(str, Enum):
    APPLICATION_STATS = "application.stats"
    ENDPOINT_RECOVER = "endpoint.recover"
    ENDPOINT_REPLAY = "endpoint.replay"
    EVENT_TYPE_AGGREGATE = "event-type.aggregate"
    MESSAGE_BROADCAST = "message.broadcast"
    SDK_GENERATE = "sdk.generate"

    def __str__(self) -> str:
        return str(self.value)
