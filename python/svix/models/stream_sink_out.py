# this file is @generated
import typing as t
from datetime import datetime

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
from .sink_status import SinkStatus
from .snowflake_config import SnowflakeConfig
from .sns_config import SnsConfig
from .sqs_config import SqsConfig


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
