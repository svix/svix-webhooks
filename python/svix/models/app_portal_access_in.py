# this file is @generated
import typing as t

from .app_portal_capability import AppPortalCapability
from .application_in import ApplicationIn
from .common import BaseModel


class AppPortalAccessIn(BaseModel):
    application: t.Optional[ApplicationIn] = None
    """Optionally creates a new application while generating the access link.

    If the application id or uid that is used in the path already exists, this argument is ignored."""

    capabilities: t.Optional[t.List[AppPortalCapability]] = None
    """Custom capabilities attached to the token, You can combine as many capabilities as necessary.

    The `ViewBase` capability is always required

    - `ViewBase`: Basic read only permissions, does not allow the user to see the endpoint secret.

    - `ViewEndpointSecret`: Allows user to view the endpoint secret.

    - `ManageEndpointSecret`: Allows user to rotate and view the endpoint secret.

    - `ManageTransformations`: Allows user to modify the endpoint transformations.

    - `CreateAttempts`: Allows user to replay missing messages and send example messages.

    - `ManageEndpoint`: Allows user to read/modify any field or configuration of an endpoint (including secrets)"""

    expiry: t.Optional[int] = None
    """How long the token will be valid for, in seconds.

    Valid values are between 1 hour and 7 days. The default is 7 days."""

    feature_flags: t.Optional[t.List[str]] = None
    """The set of feature flags the created token will have access to."""

    read_only: t.Optional[bool] = None
    """Whether the app portal should be in read-only mode."""

    session_id: t.Optional[str] = None
    """An optional session ID to attach to the token.

    When expiring tokens with "Expire All", you can include the session ID to only expire tokens that were created with that session ID."""
