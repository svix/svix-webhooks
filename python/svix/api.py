import typing as t
from contextlib import contextmanager
from dataclasses import asdict, dataclass

from .openapi_client.api.application_api import (
    ApplicationApi,
    ApplicationIn,
    ApplicationOut,
    ListResponseApplicationOut,
)
from .openapi_client.api.authentication_api import AuthenticationApi, DashboardAccessOut
from .openapi_client.api.endpoint_api import (
    EndpointApi,
    EndpointIn,
    EndpointOut,
    EndpointSecretOut,
    ListResponseEndpointOut,
)
from .openapi_client.api.event_type_api import (
    EventTypeApi,
    EventTypeIn,
    EventTypeOut,
    EventTypeUpdate,
    ListResponseEventTypeOut,
)
from .openapi_client.api.message_api import ListResponseMessageOut, MessageApi, MessageIn, MessageOut
from .openapi_client.api.message_attempt_api import ListResponseMessageAttemptOut, MessageAttemptApi, MessageAttemptOut
from .openapi_client.api_client import ApiClient
from .openapi_client.configuration import Configuration
from .openapi_client.model.message_status import MessageStatus

DEFAULT_SERVER_URL = "https://api.svix.com"


@dataclass
class SvixOptions:
    debug: bool = False
    _test_url: t.Optional[str] = None


@dataclass
class ListOptions:
    iterator: t.Optional[str] = None
    limit: t.Optional[int] = None

    def to_dict(self) -> t.Dict[str, t.Any]:
        return {k: v for k, v in asdict(self).items() if v is not None}


@dataclass
class MessageListOptions(ListOptions):
    event_types: t.Optional[t.List[str]] = None


@dataclass
class ApplicationListOptions(ListOptions):
    pass


@dataclass
class EventTypeListOptions(ListOptions):
    pass


@dataclass
class EndpointListOptions(ListOptions):
    pass


@dataclass
class MessageAttemptListOptions(ListOptions):
    status: t.Optional[MessageStatus] = None


ApiClass = t.TypeVar(
    "ApiClass",
    bound=t.Union[AuthenticationApi, ApplicationApi, EndpointApi, EventTypeApi, MessageApi, MessageAttemptApi],
)


class ApiBase(t.Generic[ApiClass]):
    _configuration: Configuration
    _ApiClass: t.Type[ApiClass]

    def __init__(self, configuration: Configuration) -> None:
        self._configuration = configuration

    @contextmanager
    def _api(self) -> t.Generator[ApiClass, None, None]:
        with ApiClient(self._configuration) as api_client:  # type: ignore

            from . import __version__

            api_client.user_agent = f"svix-libs/{__version__}/python"

            yield t.cast(t.Any, self._ApiClass(api_client))


class Authentication(ApiBase[AuthenticationApi]):
    _ApiClass = AuthenticationApi

    def dashboard_access(self, app_id: str) -> DashboardAccessOut:
        with self._api() as api:
            return api.get_dashboard_access_api_v1_auth_dashboard_access_app_id_post(
                app_id=app_id, _check_return_type=False
            )

    def logout(self) -> None:
        with self._api() as api:
            return api.logout_api_v1_auth_logout_post(_check_return_type=False)


class Application(ApiBase[ApplicationApi]):
    _ApiClass = ApplicationApi

    def list(self, options: ApplicationListOptions = ApplicationListOptions()) -> ListResponseApplicationOut:
        with self._api() as api:
            return api.list_applications_api_v1_app_get(**options.to_dict(), _check_return_type=False)

    def create(self, application_in: ApplicationIn) -> ApplicationOut:
        with self._api() as api:
            return api.create_application_api_v1_app_post(application_in=application_in, _check_return_type=False)

    def get(self, app_id: str) -> ApplicationOut:
        with self._api() as api:
            return api.get_application_api_v1_app_app_id_get(app_id=app_id, _check_return_type=False)

    def update(self, app_id: str, application_in: ApplicationIn) -> ApplicationOut:
        with self._api() as api:
            return api.update_application_api_v1_app_app_id_put(
                app_id=app_id, application_in=application_in, _check_return_type=False
            )

    def delete(self, app_id: str) -> None:
        with self._api() as api:
            return api.delete_application_api_v1_app_app_id_delete(app_id=app_id, _check_return_type=False)


class Endpoint(ApiBase[EndpointApi]):
    _ApiClass = EndpointApi

    def list(self, app_id: str, options: EndpointListOptions = EndpointListOptions()) -> ListResponseEndpointOut:
        with self._api() as api:
            return api.list_endpoints_api_v1_app_app_id_endpoint_get(
                app_id=app_id, **options.to_dict(), _check_return_type=False
            )

    def create(self, app_id: str, endpoint_in: EndpointIn) -> EndpointOut:
        with self._api() as api:
            return api.create_endpoint_api_v1_app_app_id_endpoint_post(
                app_id, endpoint_in=endpoint_in, _check_return_type=False
            )

    def get(self, app_id: str, endpoint_id: str) -> EndpointOut:
        with self._api() as api:
            return api.get_endpoint_api_v1_app_app_id_endpoint_endpoint_id_get(
                app_id=app_id, endpoint_id=endpoint_id, _check_return_type=False
            )

    def update(self, app_id: str, endpoint_id: str, endpoint_in: EndpointIn) -> EndpointOut:
        with self._api() as api:
            return api.update_endpoint_api_v1_app_app_id_endpoint_endpoint_id_put(
                app_id=app_id, endpoint_id=endpoint_id, endpoint_in=endpoint_in, _check_return_type=False
            )

    def delete(self, app_id: str, endpoint_id: str) -> None:
        with self._api() as api:
            return api.delete_endpoint_api_v1_app_app_id_endpoint_endpoint_id_delete(
                app_id=app_id, endpoint_id=endpoint_id, _check_return_type=False
            )

    def get_secret(self, app_id: str, endpoint_id: str) -> EndpointSecretOut:
        with self._api() as api:
            return api.get_endpoint_secret_api_v1_app_app_id_endpoint_endpoint_id_secret_get(
                app_id=app_id, endpoint_id=endpoint_id, _check_return_type=False
            )


class EventType(ApiBase[EventTypeApi]):
    _ApiClass = EventTypeApi

    def list(self, options: EventTypeListOptions = EventTypeListOptions()) -> ListResponseEventTypeOut:
        with self._api() as api:
            return api.list_event_types_api_v1_event_type_get(**options.to_dict(), _check_return_type=False)

    def create(self, event_type_in: EventTypeIn) -> EventTypeOut:
        with self._api() as api:
            return api.create_event_type_api_v1_event_type_post(event_type_in=event_type_in, _check_return_type=False)

    def update(self, event_type_name: str, event_type_update: EventTypeUpdate) -> EventTypeOut:
        with self._api() as api:
            return api.update_event_type_api_v1_event_type_event_type_name_put(
                event_type_name=event_type_name, event_type_update=event_type_update, _check_return_type=False
            )

    def delete(self, event_type_name: str) -> None:
        with self._api() as api:
            return api.delete_event_type_api_v1_event_type_event_type_name_delete(
                event_type_name=event_type_name, _check_return_type=False
            )


class Message(ApiBase[MessageApi]):
    _ApiClass = MessageApi

    def list(self, app_id: str, options: MessageListOptions = MessageListOptions()) -> ListResponseMessageOut:
        with self._api() as api:
            return api.list_messages_api_v1_app_app_id_msg_get(
                app_id=app_id, **options.to_dict(), _check_return_type=False
            )

    def create(self, app_id: str, message_in: MessageIn) -> MessageOut:
        with self._api() as api:
            return api.create_message_api_v1_app_app_id_msg_post(
                app_id=app_id, message_in=message_in, _check_return_type=False
            )

    def get(self, app_id: str, msg_id: str) -> MessageOut:
        with self._api() as api:
            return api.get_message_api_v1_app_app_id_msg_msg_id_get(
                app_id=app_id, msg_id=msg_id, _check_return_type=False
            )


class MessageAttempt(ApiBase[MessageAttemptApi]):
    _ApiClass = MessageAttemptApi

    def list(
        self, app_id: str, msg_id: str, options: MessageAttemptListOptions = MessageAttemptListOptions()
    ) -> ListResponseMessageAttemptOut:
        with self._api() as api:
            return api.list_attempts_api_v1_app_app_id_msg_msg_id_attempt_get(
                app_id=app_id, msg_id=msg_id, **options.to_dict(), _check_return_type=False
            )

    def get(self, app_id: str, msg_id: str, attempt_id: str) -> MessageAttemptOut:
        with self._api() as api:
            return api.get_attempt_api_v1_app_app_id_msg_msg_id_attempt_attempt_id_get(
                app_id=app_id, msg_id=msg_id, attempt_id=attempt_id, _check_return_type=False
            )

    def resend(self, app_id: str, msg_id: str, endpoint_id: str) -> None:
        with self._api() as api:
            return api.resend_webhook_api_v1_app_app_id_msg_msg_id_endpoint_endpoint_id_resend_post(
                app_id=app_id, msg_id=msg_id, endpoint_id=endpoint_id, _check_return_type=False
            )


class Svix:
    _configuration: Configuration

    def __init__(self, auth_token: str, options: SvixOptions = SvixOptions()) -> None:
        host = options._test_url or DEFAULT_SERVER_URL
        self._configuration = Configuration(host=host, access_token=auth_token)  # type: ignore

    @property
    def authentication(self) -> Authentication:
        return Authentication(self._configuration)

    @property
    def application(self) -> Application:
        return Application(self._configuration)

    @property
    def endpoint(self) -> Endpoint:
        return Endpoint(self._configuration)

    @property
    def event_type(self) -> EventType:
        return EventType(self._configuration)

    @property
    def message(self) -> Message:
        return Message(self._configuration)

    @property
    def message_attempt(self) -> MessageAttempt:
        return MessageAttempt(self._configuration)


__all__ = [
    "ApplicationIn",
    "ApplicationOut",
    "ListResponseApplicationOut",
    "DashboardAccessOut",
    "EndpointIn",
    "EndpointOut",
    "EndpointSecretOut",
    "ListResponseEndpointOut",
    "EventTypeIn",
    "EventTypeOut",
    "EventTypeUpdate",
    "ListResponseEventTypeOut",
    "ListResponseMessageOut",
    "MessageIn",
    "MessageOut",
    "ListResponseMessageAttemptOut",
    "MessageAttemptOut",
    "MessageStatus",
    "SvixOptions",
    "ApplicationListOptions",
    "EventTypeListOptions",
    "EndpointListOptions",
    "MessageAttemptListOptions",
    "Svix",
]
