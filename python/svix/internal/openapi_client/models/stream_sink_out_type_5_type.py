from enum import Enum


class StreamSinkOutType5Type(str, Enum):
    GOOGLECLOUDSTORAGE = "googleCloudStorage"

    def __str__(self) -> str:
        return str(self.value)
