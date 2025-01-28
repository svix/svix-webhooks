# this file is @generated
import typing as t

from .common import SvixBaseModel


class OperationalWebhookEndpointHeadersOut(SvixBaseModel):
    headers: t.Dict[str, str]

    sensitive: t.List[str]
