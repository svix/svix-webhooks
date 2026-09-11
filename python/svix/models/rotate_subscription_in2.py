# this file is @generated
import typing as t

from .common import BaseModel
from .endpoint_secret_rotate_in import EndpointSecretRotateIn


class RotateSubscriptionIn2(BaseModel):
    signing_secret: t.Optional[EndpointSecretRotateIn] = None
