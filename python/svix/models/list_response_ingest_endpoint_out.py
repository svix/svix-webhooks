# this file is @generated
import typing as t

from .common import BaseModel
from .ingest_endpoint_out import IngestEndpointOut


class ListResponseIngestEndpointOut(BaseModel):
    data: t.List[IngestEndpointOut]

    done: bool

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None
