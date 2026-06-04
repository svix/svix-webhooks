# this file is @generated
import typing as t

from .common import BaseModel


class EventBridgePatchConfig(BaseModel):
    access_key_id: t.Optional[str] = None

    detail_type: t.Optional[str] = None

    event_bus_name: t.Optional[str] = None

    region: t.Optional[str] = None

    secret_access_key: t.Optional[str] = None
