# this file is @generated
import typing as t

from .common import BaseModel


class StreamEventTypePatch(BaseModel):
    description: t.Optional[str] = None

    feature_flags: t.Optional[t.List[str]] = None

    deprecated: t.Optional[bool] = None

    archived: t.Optional[bool] = None
