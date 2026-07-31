# this file is @generated
import typing as t

from .common import BaseModel
from .connector_kind import ConnectorKind
from .connector_product import ConnectorProduct


class ConnectorIn(BaseModel):
    name: str

    uid: t.Optional[str] = None
    """The Connector's UID."""

    logo: t.Optional[str] = None

    description: t.Optional[str] = None

    kind: t.Optional[ConnectorKind] = None

    instructions: t.Optional[str] = None

    allowed_event_types: t.Optional[t.List[str]] = None

    transformation: str

    feature_flags: t.Optional[t.List[str]] = None

    product_type: t.Optional[ConnectorProduct] = None
