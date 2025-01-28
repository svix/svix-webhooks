# this file is @generated
import typing as t
from datetime import datetime

from pydantic import Field

from .common import SvixBaseModel
from .message_attempt_trigger_type import MessageAttemptTriggerType
from .message_out import MessageOut
from .message_status import MessageStatus


class MessageAttemptOut(SvixBaseModel):
    endpoint_id: str = Field(alias="endpointId")
    """The ep's ID"""

    id: str
    """The attempt's ID"""

    msg: t.Optional[MessageOut] = None

    msg_id: str = Field(alias="msgId")
    """The msg's ID"""

    response: str

    response_duration_ms: int = Field(alias="responseDurationMs")
    """Response duration in milliseconds."""

    response_status_code: int = Field(alias="responseStatusCode")

    status: MessageStatus

    timestamp: datetime

    trigger_type: MessageAttemptTriggerType = Field(alias="triggerType")

    url: str
