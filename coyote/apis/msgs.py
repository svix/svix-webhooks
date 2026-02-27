# This file is @generated

from ..internal.api_common import ApiBase
from .msgs_namespace import (
    MsgsNamespace,
    MsgsNamespaceAsync,
)


class MsgsAsync(ApiBase):
    @property
    def namespace(self) -> MsgsNamespaceAsync:
        return MsgsNamespaceAsync(self._client)


class Msgs(ApiBase):
    @property
    def namespace(self) -> MsgsNamespace:
        return MsgsNamespace(self._client)
