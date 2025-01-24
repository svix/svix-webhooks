from enum import Enum


class StreamSinkPatchType0Type(str, Enum):
    AZUREBLOBSTORAGE = "azureBlobStorage"

    def __str__(self) -> str:
        return str(self.value)
