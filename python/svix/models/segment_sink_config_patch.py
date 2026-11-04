# this file is @generated
import typing as t

from .common import BaseModel


class SegmentSinkConfigPatch(BaseModel):
    host: t.Optional[str] = None

    write_key: t.Optional[str] = None
