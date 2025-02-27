# this file is @generated
import typing as t

from .common import BaseModel
from .connector_kind import ConnectorKind


class ConnectorIn(BaseModel):
    description: t.Optional[str] = None

    feature_flag: t.Optional[str] = None

    filter_types: t.Optional[t.List[str]] = None

    instructions: t.Optional[str] = None

    instructions_link: t.Optional[str] = None

    kind: t.Optional[ConnectorKind] = None

    logo: str

    name: str

    transformation: str
