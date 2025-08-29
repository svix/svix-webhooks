# this file is @generated
from enum import Enum


class AppPortalCapability(str, Enum):
    VIEW_BASE = "ViewBase"
    VIEW_ENDPOINT_SECRET = "ViewEndpointSecret"
    MANAGE_ENDPOINT_SECRET = "ManageEndpointSecret"
    MANAGE_TRANSFORMATIONS = "ManageTransformations"
    CREATE_ATTEMPTS = "CreateAttempts"
    MANAGE_ENDPOINT = "ManageEndpoint"

    def __str__(self) -> str:
        return str(self.value)
