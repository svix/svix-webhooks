# this file is @generated
import typing as t

from pydantic import ModelWrapValidatorHandler, model_validator
from typing_extensions import Self

from .azure_blob_storage_config import AzureBlobStorageConfig
from .big_query_config import BigQueryConfig
from .clickhouse_config import ClickhouseConfig
from .common import BaseModel
from .event_bridge_config import EventBridgeConfig
from .google_cloud_pub_sub_config import GoogleCloudPubSubConfig
from .google_cloud_storage_config import GoogleCloudStorageConfig
from .rabbit_mq_config import RabbitMqConfig
from .redshift_config import RedshiftConfig
from .s3_config import S3Config
from .sink_http_config import SinkHttpConfig
from .sink_otel_v1_config import SinkOtelV1Config
from .sink_status_in import SinkStatusIn
from .snowflake_config import SnowflakeConfig
from .sns_config import SnsConfig
from .sqs_config import SqsConfig


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
        t.Literal["googleCloudPubSub"],
        t.Literal["sqs"],
        t.Literal["sns"],
        t.Literal["bigQuery"],
        t.Literal["clickhouse"],
        t.Literal["eventBridge"],
        t.Literal["snowflake"],
        t.Literal["rabbitMq"],
        t.Literal["redshift"],
    ]
    config: t.Union[
        t.Dict[str, t.Any],
        AzureBlobStorageConfig,
        SinkOtelV1Config,
        SinkHttpConfig,
        S3Config,
        GoogleCloudStorageConfig,
        GoogleCloudPubSubConfig,
        SqsConfig,
        SnsConfig,
        BigQueryConfig,
        ClickhouseConfig,
        EventBridgeConfig,
        SnowflakeConfig,
        RabbitMqConfig,
        RedshiftConfig,
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
        elif output.type == "googleCloudPubSub":
            output.config = GoogleCloudPubSubConfig.model_validate(
                data.get("config", {})
            )
        elif output.type == "sqs":
            output.config = SqsConfig.model_validate(data.get("config", {}))
        elif output.type == "sns":
            output.config = SnsConfig.model_validate(data.get("config", {}))
        elif output.type == "bigQuery":
            output.config = BigQueryConfig.model_validate(data.get("config", {}))
        elif output.type == "clickhouse":
            output.config = ClickhouseConfig.model_validate(data.get("config", {}))
        elif output.type == "eventBridge":
            output.config = EventBridgeConfig.model_validate(data.get("config", {}))
        elif output.type == "snowflake":
            output.config = SnowflakeConfig.model_validate(data.get("config", {}))
        elif output.type == "rabbitMq":
            output.config = RabbitMqConfig.model_validate(data.get("config", {}))
        elif output.type == "redshift":
            output.config = RedshiftConfig.model_validate(data.get("config", {}))
        else:
            raise ValueError(f"Unexpected type `{output.type}`")
        return output
