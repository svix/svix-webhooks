# this file is @generated
import typing as t

from .common import BaseModel
from .ingest_source_out import IngestSourceOut


class ListResponseIngestSourceOut(BaseModel):
    data: t.List[IngestSourceOut]

    done: bool

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None
