# this file is @generated
from enum import Enum


class Oauth2GrantTypeInOut(str, Enum):
    CLIENT_CREDENTIALS = "clientCredentials"
    REFRESH_TOKEN = "refreshToken"

    def __str__(self) -> str:
        return str(self.value)
