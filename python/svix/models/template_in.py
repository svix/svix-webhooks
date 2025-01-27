# this file is @generated
import typing as t

from .common import SvixBaseModel
from .transformation_template_kind import TransformationTemplateKind


class TemplateIn(SvixBaseModel):
    description: t.Optional[str] = None
    feature_flag: t.Optional[str] = None
    filter_types: t.Optional[t.Set[str]] = None
    instructions: t.Optional[str] = None
    instructions_link: t.Optional[str] = None
    kind: t.Optional[TransformationTemplateKind] = None
    logo: str
    name: str
    transformation: str
