# this file is @generated
import typing as t

from pydantic import Field

from .common import SvixBaseModel
from .operational_webhook_endpoint_out import OperationalWebhookEndpointOut


class ListResponseOperationalWebhookEndpointOut(SvixBaseModel):
    data: t.List[OperationalWebhookEndpointOut]

    done: bool

    iterator: str

    prev_iterator: t.Optional[str] = Field(default=None, alias="prevIterator")
