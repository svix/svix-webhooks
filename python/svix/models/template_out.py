# this file is @generated
import typing as t
from datetime import datetime

from .common import SvixBaseModel
from .transformation_template_kind import TransformationTemplateKind


class TemplateOut(SvixBaseModel):
    created_at: datetime
    description: str
    feature_flag: t.Optional[str] = None
    filter_types: t.Optional[t.Set[str]] = None
    id: str
    instructions: str
    instructions_link: t.Optional[str] = None
    kind: TransformationTemplateKind
    logo: str
    name: str
    org_id: str
    transformation: str
    updated_at: datetime
