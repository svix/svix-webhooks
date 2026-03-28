# This file is @generated

from ..internal.api_common import ApiBase
from .admin_auth_token import (
    AdminAuthToken,
    AdminAuthTokenAsync,
)
from .admin_cluster import (
    AdminCluster,
    AdminClusterAsync,
)


class AdminAsync(ApiBase):
    @property
    def auth_token(self) -> AdminAuthTokenAsync:
        return AdminAuthTokenAsync(self._client)

    @property
    def cluster(self) -> AdminClusterAsync:
        return AdminClusterAsync(self._client)


class Admin(ApiBase):
    @property
    def auth_token(self) -> AdminAuthToken:
        return AdminAuthToken(self._client)

    @property
    def cluster(self) -> AdminCluster:
        return AdminCluster(self._client)
