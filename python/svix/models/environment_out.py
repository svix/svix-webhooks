# this file is @generated
import typing as t
from datetime import datetime

from pydantic import Field

from .common import SvixBaseModel
from .event_type_out import EventTypeOut
from .template_out import TemplateOut


class EnvironmentOut(SvixBaseModel):
    created_at: datetime = Field(alias="createdAt")

    event_types: t.List[EventTypeOut] = Field(alias="eventTypes")

    settings: t.Dict[str, t.Any]

    transformation_templates: t.List[TemplateOut] = Field(
        alias="transformationTemplates"
    )

    version: t.Optional[int] = None
