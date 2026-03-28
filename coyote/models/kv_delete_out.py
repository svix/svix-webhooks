# this file is @generated

from ..internal.base_model import BaseModel


class KvDeleteOut(BaseModel):
    success: bool
    """Whether the operation succeeded or was a noop due to pre-conditions."""
