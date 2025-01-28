# this file is @generated
import typing as t
from datetime import datetime

from pydantic import Field

from .common import SvixBaseModel
from .transformation_template_kind import TransformationTemplateKind


class TemplateOut(SvixBaseModel):
    created_at: datetime = Field(alias="createdAt")

    description: str

    feature_flag: t.Optional[str] = Field(default=None, alias="featureFlag")

    filter_types: t.Optional[t.List[str]] = Field(default=None, alias="filterTypes")

    id: str

    instructions: str

    instructions_link: t.Optional[str] = Field(default=None, alias="instructionsLink")

    kind: TransformationTemplateKind

    logo: str

    name: str

    org_id: str = Field(alias="orgId")

    transformation: str

    updated_at: datetime = Field(alias="updatedAt")
