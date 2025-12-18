# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class EndpointTransformationOut(BaseModel):
    code: t.Optional[str] = None

    enabled: t.Optional[bool] = None

    updated_at: t.Optional[datetime] = None
