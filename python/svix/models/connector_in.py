# this file is @generated
import typing as t

from .common import BaseModel
from .connector_kind import ConnectorKind
from .connector_product import ConnectorProduct


class ConnectorIn(BaseModel):
    allowed_event_types: t.Optional[t.List[str]] = None

    description: t.Optional[str] = None

    feature_flags: t.Optional[t.List[str]] = None

    instructions: t.Optional[str] = None

    kind: t.Optional[ConnectorKind] = None

    logo: t.Optional[str] = None

    name: str

    product_type: t.Optional[ConnectorProduct] = None

    transformation: str

    uid: t.Optional[str] = None
    """The Connector's UID."""
