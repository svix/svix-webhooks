# This file is @generated
from .common import ApiBase
from .management_authentication import (
    ManagementAuthentication,
    ManagementAuthenticationAsync,
)


class ManagementAsync(ApiBase):
    @property
    def authentication(self) -> ManagementAuthenticationAsync:
        return ManagementAuthenticationAsync(self._client)


class Management(ApiBase):
    @property
    def authentication(self) -> ManagementAuthentication:
        return ManagementAuthentication(self._client)
