# this file is @generated
import typing as t

from pydantic import Field

from .common import SvixBaseModel
from .transformation_template_kind import TransformationTemplateKind


class TemplateIn(SvixBaseModel):
    description: t.Optional[str] = None

    feature_flag: t.Optional[str] = Field(default=None, alias="featureFlag")

    filter_types: t.Optional[t.List[str]] = Field(default=None, alias="filterTypes")

    instructions: t.Optional[str] = None

    instructions_link: t.Optional[str] = Field(default=None, alias="instructionsLink")

    kind: t.Optional[TransformationTemplateKind] = None

    logo: str

    name: str

    transformation: str
