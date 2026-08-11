# this file is @generated

from .common import BaseModel


class EventBridgeConfigOut(BaseModel):
    event_bus_name: str

    detail_type: str

    access_key_id: str

    region: str
