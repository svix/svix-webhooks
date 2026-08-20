# this file is @generated
import typing as t

from .bulk_expunge_status import BulkExpungeStatus
from .common import BaseModel


class BulkExpungeContentsOut(BaseModel):
    results: t.Dict[str, BulkExpungeStatus]
    """Results of expunging (by ID)"""
