# this file is @generated
import typing as t

from pydantic import Field

from .common import SvixBaseModel


class OperationalWebhookEndpointUpdate(SvixBaseModel):
    description: t.Optional[str] = None

    disabled: t.Optional[bool] = None

    filter_types: t.Optional[t.List[str]] = Field(default=None, alias="filterTypes")

    metadata: t.Optional[t.Dict[str, str]] = None

    rate_limit: t.Optional[int] = Field(default=None, alias="rateLimit")

    uid: t.Optional[str] = None
    """Optional unique identifier for the endpoint."""

    url: str
