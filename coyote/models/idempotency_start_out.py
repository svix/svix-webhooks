# this file is @generated
import typing as t
from typing_extensions import Self
from pydantic import ModelWrapValidatorHandler, model_validator
from ..internal.base_model import BaseModel


from .idempotency_completed import IdempotencyCompleted


class IdempotencyStartOut(BaseModel):
    status: t.Union[t.Literal["started"], t.Literal["locked"], t.Literal["completed"]]
    data: t.Union[t.Dict[str, t.Any], IdempotencyCompleted]

    @model_validator(mode="wrap")
    @classmethod
    def validate_model(
        cls, data: t.Any, handler: ModelWrapValidatorHandler[Self]
    ) -> Self:
        if "data" not in data:
            data["data"] = {}
        output = handler(data)
        if output.type == "started":
            output.data = data.get("data", {})
        elif output.type == "locked":
            output.data = data.get("data", {})
        elif output.type == "completed":
            output.data = IdempotencyCompleted.model_validate(data.get("data", {}))
        else:
            raise ValueError(f"Unexpected type `{output.type}`")
        return output
