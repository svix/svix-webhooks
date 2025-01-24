from enum import Enum


class EndpointCreatedEventType(str, Enum):
    ENDPOINT_CREATED = "endpoint.created"

    def __str__(self) -> str:
        return str(self.value)
