# this file is @generated
import typing as t

from .common import BaseModel


class BulkExpungeContentsIn(BaseModel):
    ids: t.Optional[t.List[str]] = None
    """Message ID or UID to delete"""
