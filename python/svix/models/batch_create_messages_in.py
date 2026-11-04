# this file is @generated
import typing as t

from .batch_message_in import BatchMessageIn
from .common import BaseModel


class BatchCreateMessagesIn(BaseModel):
    messages: t.List[BatchMessageIn]
