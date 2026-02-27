# this file is @generated
import typing as t
from typing_extensions import Self
from pydantic import ModelWrapValidatorHandler, model_validator
from ..internal.base_model import BaseModel


from .rate_limiter_fixed_window_config import RateLimiterFixedWindowConfig
from .rate_limiter_token_bucket_config import RateLimiterTokenBucketConfig


class RateLimiterCheckIn(BaseModel):
    key: str

    tokens: t.Optional[int] = None
    """Number of tokens to consume (default: 1)"""

    method: t.Union[t.Literal["token_bucket"], t.Literal["fixed_window"]]
    config: t.Union[RateLimiterTokenBucketConfig, RateLimiterFixedWindowConfig]

    @model_validator(mode="wrap")
    @classmethod
    def validate_model(
        cls, data: t.Any, handler: ModelWrapValidatorHandler[Self]
    ) -> Self:
        if "config" not in data:
            data["config"] = {}
        output = handler(data)
        if output.type == "token_bucket":
            output.config = RateLimiterTokenBucketConfig.model_validate(
                data.get("config", {})
            )
        elif output.type == "fixed_window":
            output.config = RateLimiterFixedWindowConfig.model_validate(
                data.get("config", {})
            )
        else:
            raise ValueError(f"Unexpected type `{output.type}`")
        return output
