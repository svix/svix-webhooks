from enum import Enum


class TransformationHttpMethod(str, Enum):
    PATCH = "PATCH"
    POST = "POST"
    PUT = "PUT"

    def __str__(self) -> str:
        return str(self.value)
