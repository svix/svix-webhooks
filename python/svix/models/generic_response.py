# this file is @generated

from .common import BaseModel


class GenericResponse(BaseModel):
    content_type: str

    body: str
