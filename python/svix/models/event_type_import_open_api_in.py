# this file is @generated
import typing as t

from .common import SvixBaseModel


class EventTypeImportOpenApiIn(SvixBaseModel):
    dry_run: t.Optional[bool] = None
    """If `true`, return the event types that would be modified without actually modifying them."""
    replace_all: t.Optional[bool] = None
    """If `true`, all existing event types that are not in the spec will be archived."""
    spec: t.Optional[t.Dict[str, t.Any]] = None
    """A pre-parsed JSON spec."""
    spec_raw: t.Optional[str] = None
    """A string, parsed by the server as YAML or JSON."""
