# this file is @generated
import typing as t

from .common import BaseModel
from .operational_webhook_endpoint_out import OperationalWebhookEndpointOut


class ListResponseOperationalWebhookEndpointOut(BaseModel):
    data: t.List[OperationalWebhookEndpointOut]

    done: bool

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None
