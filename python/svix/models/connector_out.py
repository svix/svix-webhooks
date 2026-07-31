# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel
from .connector_kind import ConnectorKind
from .connector_product import ConnectorProduct


class ConnectorOut(BaseModel):
    id: str
    """The Connector's ID."""

    org_id: str
    """The Environment's ID."""

    uid: t.Optional[str] = None
    """The Connector's UID."""

    kind: ConnectorKind

    name: str

    logo: t.Optional[str] = None

    description: str

    instructions: str

    allowed_event_types: t.Optional[t.List[str]] = None

    transformation: str

    created_at: datetime

    updated_at: datetime

    transformation_updated_at: datetime

    feature_flags: t.Optional[t.List[str]] = None

    product_type: ConnectorProduct
