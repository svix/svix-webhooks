from enum import Enum


class StreamSinkPatchType5Type(str, Enum):
    GOOGLECLOUDSTORAGE = "googleCloudStorage"

    def __str__(self) -> str:
        return str(self.value)
