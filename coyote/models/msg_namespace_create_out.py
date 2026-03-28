# this file is @generated
from datetime import datetime

from ..internal.base_model import BaseModel

from .retention import Retention


class MsgNamespaceCreateOut(BaseModel):
    name: str

    retention: Retention

    created: datetime

    updated: datetime
