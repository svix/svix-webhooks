# this file is @generated
from enum import Enum


class ElasticSearchOperation(str, Enum):
    """Operation for bulk indexing.

    `index` will do an upsert for the given document ID, and `create` will insert it
    (and do nothing on conflict). Only `create` is supported for Data Stream targets."""

    INDEX = "index"
    CREATE = "create"

    def __str__(self) -> str:
        return str(self.value)
