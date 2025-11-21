# this file is @generated
from enum import Enum


class ConnectorProduct(str, Enum):
    DISPATCH = "Dispatch"
    STREAM = "Stream"

    def __str__(self) -> str:
        return str(self.value)
