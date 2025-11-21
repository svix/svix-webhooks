# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel
from .connector_kind import ConnectorKind
from .connector_product import ConnectorProduct


class ConnectorOut(BaseModel):
    allowed_event_types: t.Optional[t.List[str]] = None

    created_at: datetime

    description: str

    feature_flags: t.Optional[t.List[str]] = None

    id: str
    """The Connector's ID."""

    instructions: str

    kind: ConnectorKind

    logo: t.Optional[str] = None

    name: str

    org_id: str
    """The Environment's ID."""

    product_type: ConnectorProduct

    transformation: str

    uid: t.Optional[str] = None
    """The Connector's UID."""

    updated_at: datetime
