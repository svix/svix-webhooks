# this file is @generated
import typing as t

from .common import BaseModel


class PostHogConfigPatch(BaseModel):
    domain: t.Optional[str] = None

    project_token: t.Optional[str] = None
