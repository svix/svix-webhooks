# this file is @generated
import typing as t

from .common import BaseModel


class GoogleCloudPubSubPatchConfig(BaseModel):
    project_id: t.Optional[str] = None

    topic_id: t.Optional[str] = None

    credentials: t.Optional[str] = None
