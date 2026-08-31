# this file is @generated
import typing as t
from datetime import datetime

from pydantic import ModelWrapValidatorHandler, model_validator
from typing_extensions import Self

from .azure_blob_storage_config_out import AzureBlobStorageConfigOut
from .big_query_config_out import BigQueryConfigOut
from .clickhouse_config_out import ClickhouseConfigOut
from .common import BaseModel
from .event_bridge_config_out import EventBridgeConfigOut
from .google_cloud_pub_sub_config_out import GoogleCloudPubSubConfigOut
from .google_cloud_storage_config_out import GoogleCloudStorageConfigOut
from .otel_tracing_config_out import OtelTracingConfigOut
from .postgres_config_out import PostgresConfigOut
from .rabbit_mq_config_out import RabbitMqConfigOut
from .redshift_config_out import RedshiftConfigOut
from .s3_config_out import S3ConfigOut
from .sink_http_config_out import SinkHttpConfigOut
from .sink_status import SinkStatus
from .snowflake_config_out import SnowflakeConfigOut
from .sns_config_out import SnsConfigOut
from .sqs_config_out import SqsConfigOut


class StreamSinkOut(BaseModel):
    id: str
    """The sink's ID."""

    uid: t.Optional[str] = None
    """The sink's UID."""

    status: SinkStatus

    current_iterator: str

    failure_reason: t.Optional[str] = None

    created_at: datetime

    updated_at: datetime

    batch_size: int

    max_wait_secs: int

    event_types: t.Optional[t.List[str]] = None

    channels: t.Optional[t.List[str]] = None

    next_retry_at: t.Optional[datetime] = None

    metadata: t.Dict[str, str]

    type: t.Union[
        t.Literal["poller"],
        t.Literal["azureBlobStorage"],
        t.Literal["otelTracing"],
        t.Literal["http"],
        t.Literal["amazonS3"],
        t.Literal["snowflake"],
        t.Literal["googleCloudStorage"],
        t.Literal["googleCloudPubSub"],
        t.Literal["redshift"],
        t.Literal["bigQuery"],
        t.Literal["clickhouse"],
        t.Literal["rabbitMq"],
        t.Literal["sqs"],
        t.Literal["eventBridge"],
        t.Literal["sns"],
        t.Literal["postgres"],
    ]
    config: t.Union[
        t.Dict[str, t.Any],
        AzureBlobStorageConfigOut,
        OtelTracingConfigOut,
        SinkHttpConfigOut,
        S3ConfigOut,
        SnowflakeConfigOut,
        GoogleCloudStorageConfigOut,
        GoogleCloudPubSubConfigOut,
        RedshiftConfigOut,
        BigQueryConfigOut,
        ClickhouseConfigOut,
        RabbitMqConfigOut,
        SqsConfigOut,
        EventBridgeConfigOut,
        SnsConfigOut,
        PostgresConfigOut,
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
            output.config = AzureBlobStorageConfigOut.model_validate(
                data.get("config", {})
            )
        elif output.type == "otelTracing":
            output.config = OtelTracingConfigOut.model_validate(data.get("config", {}))
        elif output.type == "http":
            output.config = SinkHttpConfigOut.model_validate(data.get("config", {}))
        elif output.type == "amazonS3":
            output.config = S3ConfigOut.model_validate(data.get("config", {}))
        elif output.type == "snowflake":
            output.config = SnowflakeConfigOut.model_validate(data.get("config", {}))
        elif output.type == "googleCloudStorage":
            output.config = GoogleCloudStorageConfigOut.model_validate(
                data.get("config", {})
            )
        elif output.type == "googleCloudPubSub":
            output.config = GoogleCloudPubSubConfigOut.model_validate(
                data.get("config", {})
            )
        elif output.type == "redshift":
            output.config = RedshiftConfigOut.model_validate(data.get("config", {}))
        elif output.type == "bigQuery":
            output.config = BigQueryConfigOut.model_validate(data.get("config", {}))
        elif output.type == "clickhouse":
            output.config = ClickhouseConfigOut.model_validate(data.get("config", {}))
        elif output.type == "rabbitMq":
            output.config = RabbitMqConfigOut.model_validate(data.get("config", {}))
        elif output.type == "sqs":
            output.config = SqsConfigOut.model_validate(data.get("config", {}))
        elif output.type == "eventBridge":
            output.config = EventBridgeConfigOut.model_validate(data.get("config", {}))
        elif output.type == "sns":
            output.config = SnsConfigOut.model_validate(data.get("config", {}))
        elif output.type == "postgres":
            output.config = PostgresConfigOut.model_validate(data.get("config", {}))
        else:
            raise ValueError(f"Unexpected type `{output.type}`")
        return output
