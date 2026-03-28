# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel


class ClusterRemoveNodeIn(BaseModel):
    node_id: str = Field(alias="node_id")
