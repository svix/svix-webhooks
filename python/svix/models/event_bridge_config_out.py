# this file is @generated
import typing as t

from .common import BaseModel


class EventBridgeConfigOut(BaseModel):
    event_bus_name: str

    detail_type: str

    access_key_id: t.Optional[str] = None

    role_arn: t.Optional[str] = None

    external_id: t.Optional[str] = None

    region: str
