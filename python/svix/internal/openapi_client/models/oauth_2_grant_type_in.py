from enum import Enum


class Oauth2GrantTypeIn(str, Enum):
    CLIENTCREDENTIALS = "clientCredentials"
    REFRESHTOKEN = "refreshToken"

    def __str__(self) -> str:
        return str(self.value)
