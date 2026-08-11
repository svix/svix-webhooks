# this file is @generated
import typing as t

from pydantic import ModelWrapValidatorHandler, model_validator
from typing_extensions import Self

from .azure_blob_storage_config_in import AzureBlobStorageConfigIn
from .big_query_config_in import BigQueryConfigIn
from .clickhouse_config_in import ClickhouseConfigIn
from .common import BaseModel
from .event_bridge_config_in import EventBridgeConfigIn
from .google_cloud_pub_sub_config_in import GoogleCloudPubSubConfigIn
from .google_cloud_storage_config_in import GoogleCloudStorageConfigIn
from .rabbit_mq_config_in import RabbitMqConfigIn
from .redshift_config_in import RedshiftConfigIn
from .s3_config_in import S3ConfigIn
from .sink_http_config_in import SinkHttpConfigIn
from .sink_otel_tracing_config_in import SinkOtelTracingConfigIn
from .sink_status_in import SinkStatusIn
from .snowflake_config_in import SnowflakeConfigIn
from .sns_config_in import SnsConfigIn
from .sqs_config_in import SqsConfigIn


class StreamSinkIn(BaseModel):
    uid: t.Optional[str] = None
    """An optional unique identifier for the sink."""

    status: t.Optional[SinkStatusIn] = None
    """Whether the sink will receive events.

    If the sink is `enabled`, any events posted to the stream will be dispatched to the Sink in the same order that events were posted to the stream.

    If the sink is `disabled`, events will not be dispatched to the sink until the sink is reenabled."""

    batch_size: t.Optional[int] = None
    """How many events will be batched in a request to the Sink."""

    max_wait_secs: t.Optional[int] = None
    """How long to wait before a batch of events is sent, if the `batchSize` is not reached.

    For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, we will send a request after 10 seconds or 100 events, whichever comes first.

    Note that we will never send an empty batch of events to the Sink."""

    event_types: t.Optional[t.List[str]] = None
    """A list of event types that filter which events are dispatched to the Sink. An empty list (or null) will not filter out any events."""

    metadata: t.Optional[t.Dict[str, str]] = None

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
        AzureBlobStorageConfigIn,
        SinkOtelTracingConfigIn,
        SinkHttpConfigIn,
        S3ConfigIn,
        GoogleCloudStorageConfigIn,
        GoogleCloudPubSubConfigIn,
        SqsConfigIn,
        SnsConfigIn,
        BigQueryConfigIn,
        ClickhouseConfigIn,
        EventBridgeConfigIn,
        SnowflakeConfigIn,
        RabbitMqConfigIn,
        RedshiftConfigIn,
    ]

    @model_validator(mode="wrap")
    @classmethod
    def validate_model(
        cls, data: t.Any, handler: ModelWrapValidatorHandler[Self]
    ) -> Self:
        if isinstance(data, cls):
            return handler(data)
        if "config" not in data:
            data["config"] = {}
        output = handler(data)
        if output.type == "poller":
            output.config = data.get("config", {})
        elif output.type == "azureBlobStorage":
            output.config = AzureBlobStorageConfigIn.model_validate(
                data.get("config", {})
            )
        elif output.type == "otelTracing":
            output.config = SinkOtelTracingConfigIn.model_validate(
                data.get("config", {})
            )
        elif output.type == "http":
            output.config = SinkHttpConfigIn.model_validate(data.get("config", {}))
        elif output.type == "amazonS3":
            output.config = S3ConfigIn.model_validate(data.get("config", {}))
        elif output.type == "googleCloudStorage":
            output.config = GoogleCloudStorageConfigIn.model_validate(
                data.get("config", {})
            )
        elif output.type == "googleCloudPubSub":
            output.config = GoogleCloudPubSubConfigIn.model_validate(
                data.get("config", {})
            )
        elif output.type == "sqs":
            output.config = SqsConfigIn.model_validate(data.get("config", {}))
        elif output.type == "sns":
            output.config = SnsConfigIn.model_validate(data.get("config", {}))
        elif output.type == "bigQuery":
            output.config = BigQueryConfigIn.model_validate(data.get("config", {}))
        elif output.type == "clickhouse":
            output.config = ClickhouseConfigIn.model_validate(data.get("config", {}))
        elif output.type == "eventBridge":
            output.config = EventBridgeConfigIn.model_validate(data.get("config", {}))
        elif output.type == "snowflake":
            output.config = SnowflakeConfigIn.model_validate(data.get("config", {}))
        elif output.type == "rabbitMq":
            output.config = RabbitMqConfigIn.model_validate(data.get("config", {}))
        elif output.type == "redshift":
            output.config = RedshiftConfigIn.model_validate(data.get("config", {}))
        else:
            raise ValueError(f"Unexpected type `{output.type}`")
        return output
