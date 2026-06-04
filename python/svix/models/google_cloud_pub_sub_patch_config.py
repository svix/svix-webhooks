# this file is @generated
import typing as t

from .common import BaseModel


class GoogleCloudPubSubPatchConfig(BaseModel):
    credentials: t.Optional[str] = None

    project_id: t.Optional[str] = None

    topic_id: t.Optional[str] = None
