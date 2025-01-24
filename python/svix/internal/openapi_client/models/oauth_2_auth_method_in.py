from enum import Enum


class Oauth2AuthMethodIn(str, Enum):
    CLIENTSECRETBASIC = "clientSecretBasic"
    CLIENTSECRETJWT = "clientSecretJwt"
    CLIENTSECRETPOST = "clientSecretPost"

    def __str__(self) -> str:
        return str(self.value)
