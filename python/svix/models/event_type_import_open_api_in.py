# this file is @generated
import typing as t

from .common import BaseModel


class EventTypeImportOpenApiIn(BaseModel):
    """Import a list of event types from webhooks defined in an OpenAPI spec.

    The OpenAPI spec can be specified as either `spec` given the spec as a JSON object, or as `specRaw` (a `string`) which will be parsed as YAML or JSON by the server. Sending neither or both is invalid, resulting in a `400` **Bad Request**."""

    dry_run: t.Optional[bool] = None
    """If `true`, return the event types that would be modified without actually modifying them."""

    replace_all: t.Optional[bool] = None
    """If `true`, all existing event types that are not in the spec will be archived."""

    spec: t.Optional[t.Dict[str, t.Any]] = None
    """A pre-parsed JSON spec."""

    spec_raw: t.Optional[str] = None
    """A string, parsed by the server as YAML or JSON."""
