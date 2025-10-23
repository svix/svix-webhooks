# this file is @generated
import typing as t

from .common import BaseModel
from .connector_kind import ConnectorKind


class ConnectorUpdate(BaseModel):
    allowed_event_types: t.Optional[t.List[str]] = None

    description: t.Optional[str] = None

    feature_flags: t.Optional[t.List[str]] = None

    instructions: t.Optional[str] = None

    kind: t.Optional[ConnectorKind] = None

    logo: t.Optional[str] = None

    name: t.Optional[str] = None

    transformation: str
