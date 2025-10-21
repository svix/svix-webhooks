# this file is @generated
import typing as t
from datetime import datetime

from pydantic import ModelWrapValidatorHandler, model_validator
from typing_extensions import Self

from .azure_blob_storage_config import AzureBlobStorageConfig
from .common import BaseModel
from .google_cloud_storage_config import GoogleCloudStorageConfig
from .s3_config import S3Config
from .sink_http_config import SinkHttpConfig
from .sink_otel_v1_config import SinkOtelV1Config
from .sink_status import SinkStatus


class StreamSinkOut(BaseModel):
    batch_size: int

    created_at: datetime

    current_iterator: str

    event_types: t.Optional[t.List[str]] = None

    failure_reason: t.Optional[str] = None

    id: str
    """The sink's ID."""

    max_wait_secs: int

    metadata: t.Dict[str, str]

    next_retry_at: t.Optional[datetime] = None

    status: SinkStatus

    uid: t.Optional[str] = None
    """The sink's UID."""

    updated_at: datetime

    type: t.Union[
        t.Literal["poller"],
        t.Literal["azureBlobStorage"],
        t.Literal["otelTracing"],
        t.Literal["http"],
        t.Literal["amazonS3"],
        t.Literal["googleCloudStorage"],
    ]
    config: t.Union[
        t.Dict[str, t.Any],
        AzureBlobStorageConfig,
        SinkOtelV1Config,
        SinkHttpConfig,
        S3Config,
        GoogleCloudStorageConfig,
    ]

    @model_validator(mode="wrap")
    @classmethod
    def validate_model(
        cls, data: t.Any, handler: ModelWrapValidatorHandler[Self]
    ) -> Self:
        if "config" not in data:
            data["config"] = {}
        output = handler(data)
        if output.type == "poller":
            output.config = data.get("config", {})
        elif output.type == "azureBlobStorage":
            output.config = AzureBlobStorageConfig.model_validate(
                data.get("config", {})
            )
        elif output.type == "otelTracing":
            output.config = SinkOtelV1Config.model_validate(data.get("config", {}))
        elif output.type == "http":
            output.config = SinkHttpConfig.model_validate(data.get("config", {}))
        elif output.type == "amazonS3":
            output.config = S3Config.model_validate(data.get("config", {}))
        elif output.type == "googleCloudStorage":
            output.config = GoogleCloudStorageConfig.model_validate(
                data.get("config", {})
            )
        else:
            raise ValueError(f"Unexpected type `{output.type}`")
        return output
