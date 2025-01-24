from enum import IntEnum


class StatusCodeClass(IntEnum):
    VALUE_0 = 0
    VALUE_100 = 100
    VALUE_200 = 200
    VALUE_300 = 300
    VALUE_400 = 400
    VALUE_500 = 500

    def __str__(self) -> str:
        return str(self.value)
