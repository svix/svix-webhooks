# this file is @generated
import typing as t

from .common import BaseModel
from .message_attempt_log import MessageAttemptLog


class MessageAttemptLogEvent(BaseModel):
    """Sent after message attempts are made. Contains metadata about message attempts and their results. In order to reduce the frequency of webhooks, these are sent in batches periodically."""

    data: t.List[MessageAttemptLog]

    type: str
