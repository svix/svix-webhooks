# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class RecoverIn(BaseModel):
    since: datetime

    until: t.Optional[datetime] = None
