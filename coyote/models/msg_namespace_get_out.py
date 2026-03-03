# this file is @generated
from pydantic import Field
from datetime import datetime

from ..internal.base_model import BaseModel

from .retention import Retention
from .storage_type import StorageType


class MsgNamespaceGetOut(BaseModel):
    name: str

    retention: Retention

    storage_type: StorageType = Field(alias="storage_type")

    created: datetime

    updated: datetime
