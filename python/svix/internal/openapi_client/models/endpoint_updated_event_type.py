from enum import Enum


class EndpointUpdatedEventType(str, Enum):
    ENDPOINT_UPDATED = "endpoint.updated"

    def __str__(self) -> str:
        return str(self.value)
