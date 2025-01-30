# this file is @generated
import typing as t

from .common import BaseModel
from .integration_out import IntegrationOut


class ListResponseIntegrationOut(BaseModel):
    data: t.List[IntegrationOut]

    done: bool

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None
