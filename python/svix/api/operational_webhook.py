# This file is @generated
from .common import ApiBase
from .operational_webhook_endpoint import (
    OperationalWebhookEndpoint,
    OperationalWebhookEndpointAsync,
)


class OperationalWebhookAsync(ApiBase):
    @property
    def endpoint(self) -> OperationalWebhookEndpointAsync:
        return OperationalWebhookEndpointAsync(self._client)


class OperationalWebhook(ApiBase):
    @property
    def endpoint(self) -> OperationalWebhookEndpoint:
        return OperationalWebhookEndpoint(self._client)
