# this file is @generated
import typing as t

from .common import BaseModel
from .elastic_search_operation import ElasticSearchOperation


class ElasticSearchConfigIn(BaseModel):
    """Configuration parameters for defining an ElasticSearch/OpenSearch sink."""

    index_name: str
    """Name of the index to write to

    This can also be a data stream on sufficiently-new versions of ElasticSearch/OpenSearch"""

    url: str
    """Base URL to send indexing requests

    This should not include the /{index}/_bulk suffix."""

    api_key: t.Optional[str] = None
    """API key for authentication for ElasticSearch

    If not passed, any username:password embedded in the URL will be used. If none is passed,
    the indexing will be done unauthenticated."""

    refresh: t.Optional[bool] = None
    """If true, Elasticsearch refreshes the affected shards to make this operation visible to search."""

    operation: t.Optional[ElasticSearchOperation] = None
    """The operation to use when indexing.

    If not passed, this defaults to `create`."""
