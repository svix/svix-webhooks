# this file is @generated
import typing as t

from pydantic import ModelWrapValidatorHandler, model_validator
from typing_extensions import Self

from .common import BaseModel
from .endpoint_in import EndpointIn
from .sink_in_common import SinkInCommon


class AutoConfigSinkType(BaseModel):
    type: t.Union[t.Literal["poller"], t.Literal["http"]]
    config: t.Union[SinkInCommon, EndpointIn]

    @model_validator(mode="wrap")
    @classmethod
    def validate_model(
        cls, data: t.Any, handler: ModelWrapValidatorHandler[Self]
    ) -> Self:
        if "config" not in data:
            data["config"] = {}
        output = handler(data)
        if output.type == "poller":
            output.config = SinkInCommon.model_validate(data.get("config", {}))
        elif output.type == "http":
            output.config = EndpointIn.model_validate(data.get("config", {}))
        else:
            raise ValueError(f"Unexpected type `{output.type}`")
        return output
