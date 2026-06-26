# this file is @generated
import typing as t

from .common import BaseModel
from .poller_v2_message_out import PollerV2MessageOut


class PollerV2PollOut(BaseModel):
    data: t.List[PollerV2MessageOut]

    done: bool
