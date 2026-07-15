# This file is @generated
from .common import ApiBaseAsync, ApiBaseSync
from .operational_webhook_endpoint import (
    OperationalWebhookEndpoint,
    OperationalWebhookEndpointAsync,
)


class OperationalWebhookAsync(ApiBaseAsync):
    @property
    def endpoint(self) -> OperationalWebhookEndpointAsync:
        return OperationalWebhookEndpointAsync(self._client)


class OperationalWebhook(ApiBaseSync):
    @property
    def endpoint(self) -> OperationalWebhookEndpoint:
        return OperationalWebhookEndpoint(self._client)
