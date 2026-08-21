# this file is @generated
import typing as t

from pydantic import ModelWrapValidatorHandler, model_validator
from typing_extensions import Self

from .azure_blob_storage_config_patch import AzureBlobStorageConfigPatch
from .big_query_config_patch import BigQueryConfigPatch
from .clickhouse_config_patch import ClickhouseConfigPatch
from .common import BaseModel
from .event_bridge_config_patch import EventBridgeConfigPatch
from .google_cloud_pub_sub_config_patch import GoogleCloudPubSubConfigPatch
from .google_cloud_storage_config_patch import GoogleCloudStorageConfigPatch
from .otel_tracing_config_patch import OtelTracingConfigPatch
from .rabbit_mq_config_patch import RabbitMqConfigPatch
from .redshift_config_patch import RedshiftConfigPatch
from .s3_config_patch import S3ConfigPatch
from .sink_http_config_patch import SinkHttpConfigPatch
from .sink_status_in import SinkStatusIn
from .snowflake_config_patch import SnowflakeConfigPatch
from .sns_config_patch import SnsConfigPatch
from .sqs_config_patch import SqsConfigPatch


class StreamSinkPatch(BaseModel):
    uid: t.Optional[str] = None
    """The StreamSink's UID."""

    status: t.Optional[SinkStatusIn] = None

    batch_size: t.Optional[int] = None

    max_wait_secs: t.Optional[int] = None

    event_types: t.Optional[t.List[str]] = None

    channels: t.Optional[t.List[str]] = None

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
        AzureBlobStorageConfigPatch,
        OtelTracingConfigPatch,
        SinkHttpConfigPatch,
        S3ConfigPatch,
        GoogleCloudStorageConfigPatch,
        GoogleCloudPubSubConfigPatch,
        SqsConfigPatch,
        SnsConfigPatch,
        BigQueryConfigPatch,
        ClickhouseConfigPatch,
        EventBridgeConfigPatch,
        SnowflakeConfigPatch,
        RabbitMqConfigPatch,
        RedshiftConfigPatch,
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
            output.config = AzureBlobStorageConfigPatch.model_validate(
                data.get("config", {})
            )
        elif output.type == "otelTracing":
            output.config = OtelTracingConfigPatch.model_validate(
                data.get("config", {})
            )
        elif output.type == "http":
            output.config = SinkHttpConfigPatch.model_validate(data.get("config", {}))
        elif output.type == "amazonS3":
            output.config = S3ConfigPatch.model_validate(data.get("config", {}))
        elif output.type == "googleCloudStorage":
            output.config = GoogleCloudStorageConfigPatch.model_validate(
                data.get("config", {})
            )
        elif output.type == "googleCloudPubSub":
            output.config = GoogleCloudPubSubConfigPatch.model_validate(
                data.get("config", {})
            )
        elif output.type == "sqs":
            output.config = SqsConfigPatch.model_validate(data.get("config", {}))
        elif output.type == "sns":
            output.config = SnsConfigPatch.model_validate(data.get("config", {}))
        elif output.type == "bigQuery":
            output.config = BigQueryConfigPatch.model_validate(data.get("config", {}))
        elif output.type == "clickhouse":
            output.config = ClickhouseConfigPatch.model_validate(data.get("config", {}))
        elif output.type == "eventBridge":
            output.config = EventBridgeConfigPatch.model_validate(
                data.get("config", {})
            )
        elif output.type == "snowflake":
            output.config = SnowflakeConfigPatch.model_validate(data.get("config", {}))
        elif output.type == "rabbitMq":
            output.config = RabbitMqConfigPatch.model_validate(data.get("config", {}))
        elif output.type == "redshift":
            output.config = RedshiftConfigPatch.model_validate(data.get("config", {}))
        else:
            raise ValueError(f"Unexpected type `{output.type}`")
        return output
