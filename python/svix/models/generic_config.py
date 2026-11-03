# this file is @generated
import typing as t

from .common import BaseModel
from .generic_response import GenericResponse


class GenericConfig(BaseModel):
    response: t.Optional[GenericResponse] = None
