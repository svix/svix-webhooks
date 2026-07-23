# this file is @generated
import typing as t

from .common import BaseModel
from .connector_kind import ConnectorKind


class ConnectorUpsertIn(BaseModel):
    name: t.Optional[str] = None

    logo: t.Optional[str] = None

    description: t.Optional[str] = None

    kind: t.Optional[ConnectorKind] = None

    instructions: t.Optional[str] = None

    allowed_event_types: t.Optional[t.List[str]] = None

    transformation: str

    feature_flags: t.Optional[t.List[str]] = None
