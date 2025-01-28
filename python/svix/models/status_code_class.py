# this file is @generated
from enum import IntEnum


class StatusCodeClass(IntEnum):
    CODE_NONE = 0
    CODE1XX = 100
    CODE2XX = 200
    CODE3XX = 300
    CODE4XX = 400
    CODE5XX = 500

    def __str__(self) -> str:
        return str(self.value)
