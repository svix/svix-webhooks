# this file is @generated
from enum import IntEnum


class StatusCodeClass(IntEnum):
    """The different classes of HTTP status codes:
    - CodeNone = 0
    - Code1xx = 100
    - Code2xx = 200
    - Code3xx = 300
    - Code4xx = 400
    - Code5xx = 500"""

    CODE_NONE = 0
    CODE1XX = 100
    CODE2XX = 200
    CODE3XX = 300
    CODE4XX = 400
    CODE5XX = 500

    def __str__(self) -> str:
        return str(self.value)
