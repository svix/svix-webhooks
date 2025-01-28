# this file is @generated
import typing as t

from .common import SvixBaseModel


class OperationalWebhookEndpointHeadersIn(SvixBaseModel):
    headers: t.Dict[str, str]
