from enum import Enum


class EndpointDeletedEventType(str, Enum):
    ENDPOINT_DELETED = "endpoint.deleted"

    def __str__(self) -> str:
        return str(self.value)
