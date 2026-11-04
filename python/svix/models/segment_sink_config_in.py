# this file is @generated

from .common import BaseModel


class SegmentSinkConfigIn(BaseModel):
    """Configuration parameters for defining a Segment sink."""

    host: str
    """The Segment ingestion host.

    This is https://api.segment.io for US workspaces, https://events.eu1.segmentapis.com
    for EU workspaces, or your custom domain. Events are sent to the `/v1/batch` endpoint
    under this host."""

    write_key: str
    """The Segment source write key.

    Sent as HTTP Basic auth (the write key as the username, with an empty password) on each
    `/v1/batch` request."""
