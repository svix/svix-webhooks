# this file is @generated
import typing as t

from pydantic import ModelWrapValidatorHandler, model_validator
from typing_extensions import Self

from .amazon_s3_patch_config import AmazonS3PatchConfig
from .azure_blob_storage_patch_config import AzureBlobStoragePatchConfig
from .big_query_patch_config import BigQueryPatchConfig
from .clickhouse_patch_config import ClickhousePatchConfig
from .common import BaseModel
from .event_bridge_patch_config import EventBridgePatchConfig
from .google_cloud_pub_sub_patch_config import GoogleCloudPubSubPatchConfig
from .google_cloud_storage_patch_config import GoogleCloudStoragePatchConfig
from .http_patch_config import HttpPatchConfig
from .otel_tracing_patch_config import OtelTracingPatchConfig
from .rabbit_mq_patch_config import RabbitMqPatchConfig
from .redshift_patch_config import RedshiftPatchConfig
from .sink_status_in import SinkStatusIn
from .snowflake_patch_config import SnowflakePatchConfig
from .sns_patch_config import SnsPatchConfig
from .sqs_patch_config import SqsPatchConfig


class StreamSinkPatch(BaseModel):
    batch_size: t.Optional[int] = None

    event_types: t.Optional[t.List[str]] = None

    max_wait_secs: t.Optional[int] = None

    metadata: t.Optional[t.Dict[str, str]] = None

    status: t.Optional[SinkStatusIn] = None

    uid: t.Optional[str] = None
    """The StreamSink's UID."""

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
        AzureBlobStoragePatchConfig,
        OtelTracingPatchConfig,
        HttpPatchConfig,
        AmazonS3PatchConfig,
        GoogleCloudStoragePatchConfig,
        GoogleCloudPubSubPatchConfig,
        SqsPatchConfig,
        SnsPatchConfig,
        BigQueryPatchConfig,
        ClickhousePatchConfig,
        EventBridgePatchConfig,
        SnowflakePatchConfig,
        RabbitMqPatchConfig,
        RedshiftPatchConfig,
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
            output.config = AzureBlobStoragePatchConfig.model_validate(
                data.get("config", {})
            )
        elif output.type == "otelTracing":
            output.config = OtelTracingPatchConfig.model_validate(
                data.get("config", {})
            )
        elif output.type == "http":
            output.config = HttpPatchConfig.model_validate(data.get("config", {}))
        elif output.type == "amazonS3":
            output.config = AmazonS3PatchConfig.model_validate(data.get("config", {}))
        elif output.type == "googleCloudStorage":
            output.config = GoogleCloudStoragePatchConfig.model_validate(
                data.get("config", {})
            )
        elif output.type == "googleCloudPubSub":
            output.config = GoogleCloudPubSubPatchConfig.model_validate(
                data.get("config", {})
            )
        elif output.type == "sqs":
            output.config = SqsPatchConfig.model_validate(data.get("config", {}))
        elif output.type == "sns":
            output.config = SnsPatchConfig.model_validate(data.get("config", {}))
        elif output.type == "bigQuery":
            output.config = BigQueryPatchConfig.model_validate(data.get("config", {}))
        elif output.type == "clickhouse":
            output.config = ClickhousePatchConfig.model_validate(data.get("config", {}))
        elif output.type == "eventBridge":
            output.config = EventBridgePatchConfig.model_validate(
                data.get("config", {})
            )
        elif output.type == "snowflake":
            output.config = SnowflakePatchConfig.model_validate(data.get("config", {}))
        elif output.type == "rabbitMq":
            output.config = RabbitMqPatchConfig.model_validate(data.get("config", {}))
        elif output.type == "redshift":
            output.config = RedshiftPatchConfig.model_validate(data.get("config", {}))
        else:
            raise ValueError(f"Unexpected type `{output.type}`")
        return output
