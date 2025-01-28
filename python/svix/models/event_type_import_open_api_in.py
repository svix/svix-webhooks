# this file is @generated
import typing as t

from pydantic import Field

from .common import SvixBaseModel


class EventTypeImportOpenApiIn(SvixBaseModel):
    dry_run: t.Optional[bool] = Field(default=None, alias="dryRun")
    """If `true`, return the event types that would be modified without actually modifying them."""

    replace_all: t.Optional[bool] = Field(default=None, alias="replaceAll")
    """If `true`, all existing event types that are not in the spec will be archived."""

    spec: t.Optional[t.Dict[str, t.Any]] = None
    """A pre-parsed JSON spec."""

    spec_raw: t.Optional[str] = Field(default=None, alias="specRaw")
    """A string, parsed by the server as YAML or JSON."""
