# this file is @generated
import typing as t

from pydantic import ModelWrapValidatorHandler, model_validator
from typing_extensions import Self

from .azure_blob_storage_config import AzureBlobStorageConfig
from .common import BaseModel
from .google_cloud_storage_config import GoogleCloudStorageConfig
from .s3_config import S3Config
from .sink_http_config import SinkHttpConfig
from .sink_otel_v1_config import SinkOtelV1Config
from .sink_status_in import SinkStatusIn


class StreamSinkIn(BaseModel):
    batch_size: t.Optional[int] = None
    """How many events will be batched in a request to the Sink."""

    event_types: t.Optional[t.List[str]] = None
    """A list of event types that filter which events are dispatched to the Sink. An empty list (or null) will not filter out any events."""

    max_wait_secs: t.Optional[int] = None
    """How long to wait before a batch of events is sent, if the `batchSize` is not reached.

    For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, we will send a request after 10 seconds or 100 events, whichever comes first.

    Note that we will never send an empty batch of events to the Sink."""

    metadata: t.Optional[t.Dict[str, str]] = None

    status: t.Optional[SinkStatusIn] = None
    """Whether the sink will receive events.

    If the sink is `enabled`, any events posted to the stream will be dispatched to the Sink in the same order that events were posted to the stream.

    If the sink is `disabled`, events will not be dispatched to the sink until the sink is reenabled."""

    uid: t.Optional[str] = None
    """An optional unique identifier for the sink."""

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
