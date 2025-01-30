# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class ApplicationOut(BaseModel):
    created_at: datetime

    id: str
    """The app's ID"""

    metadata: t.Dict[str, str]

    name: str

    rate_limit: t.Optional[int] = None

    uid: t.Optional[str] = None
    """The app's UID"""

    updated_at: datetime
