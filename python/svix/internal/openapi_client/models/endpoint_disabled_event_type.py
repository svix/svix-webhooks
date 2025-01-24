from enum import Enum


class EndpointDisabledEventType(str, Enum):
    ENDPOINT_DISABLED = "endpoint.disabled"

    def __str__(self) -> str:
        return str(self.value)
