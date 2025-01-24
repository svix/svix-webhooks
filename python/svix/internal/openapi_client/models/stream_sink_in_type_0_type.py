from enum import Enum


class StreamSinkInType0Type(str, Enum):
    AZUREBLOBSTORAGE = "azureBlobStorage"

    def __str__(self) -> str:
        return str(self.value)
