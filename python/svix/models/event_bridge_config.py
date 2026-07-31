# this file is @generated
import typing as t

from .common import BaseModel


class EventBridgeConfig(BaseModel):
    event_bus_name: str
    """The name or ARN of the event bus to receive the event"""

    detail_type: t.Optional[str] = None
    """Free-form string, with a maximum of 128 characters"""

    access_key_id: str

    secret_access_key: str

    region: str
