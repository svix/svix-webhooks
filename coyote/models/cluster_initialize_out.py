# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel


class ClusterInitializeOut(BaseModel):
    cluster_id: str = Field(alias="cluster_id")
