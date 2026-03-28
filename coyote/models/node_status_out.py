# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel

from .server_state import ServerState


class NodeStatusOut(BaseModel):
    node_id: str = Field(alias="node_id")
    """A unique ID representing this node.

    This will never change unless the node is erased and reset"""

    address: str
    """The advertised inter-server (cluster) address of this node."""

    state: ServerState
    """The last known state of this node"""

    last_committed_log_index: t.Optional[int] = Field(
        default=None, alias="last_committed_log_index"
    )
    """The index of the last log applied on this node"""

    last_committed_term: t.Optional[int] = Field(
        default=None, alias="last_committed_term"
    )
    """The raft term of the last committed leadership"""
