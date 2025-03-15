# this file is @generated
import typing as t

from .common import BaseModel


class CronConfig(BaseModel):
    content_type: t.Optional[str] = None
    """Override the default content-type.

    Recommended if the payload is not JSON."""

    payload: str

    schedule: str
