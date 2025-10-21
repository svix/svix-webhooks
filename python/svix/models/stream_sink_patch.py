# this file is @generated
import typing as t

from pydantic import ModelWrapValidatorHandler, model_validator
from typing_extensions import Self

from .amazon_s3_patch_config import AmazonS3PatchConfig
from .azure_blob_storage_patch_config import AzureBlobStoragePatchConfig
from .common import BaseModel
from .google_cloud_storage_patch_config import GoogleCloudStoragePatchConfig
from .http_patch_config import HttpPatchConfig
from .otel_tracing_patch_config import OtelTracingPatchConfig
from .sink_status_in import SinkStatusIn


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
    ]
    config: t.Union[
        t.Dict[str, t.Any],
        AzureBlobStoragePatchConfig,
        OtelTracingPatchConfig,
        HttpPatchConfig,
        AmazonS3PatchConfig,
        GoogleCloudStoragePatchConfig,
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
        else:
            raise ValueError(f"Unexpected type `{output.type}`")
        return output
