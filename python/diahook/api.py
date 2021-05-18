import typing as t
from dataclasses import asdict, dataclass
from contextlib import contextmanager

from .openapi_client.api_client import ApiClient
from .openapi_client.configuration import Configuration
from .openapi_client.api.application_api import (
    ApplicationApi,
    ListResponseApplicationOut,
    ApplicationIn,
    ApplicationOut,
)
from .openapi_client.api.authentication_api import AuthenticationApi, DashboardAccessOut
from .openapi_client.api.endpoint_api import (
    EndpointApi,
    EndpointIn,
    EndpointOut,
    EndpointSecret,
    ListResponseEndpointOut,
)
from .openapi_client.api.event_type_api import EventTypeApi, EventTypeInOut, ListResponseEventTypeInOut
from .openapi_client.api.message_api import MessageApi, MessageIn, MessageOut, ListResponseMessageOut
from .openapi_client.api.message_attempt_api import (
    MessageAttemptApi,
    MessageAttemptOut,
    ListResponseMessageAttemptOut,
)
from .openapi_client.model.message_status import MessageStatus


@dataclass
class DiahookOptions:
    debug: bool = False
    _test_url: t.Optional[str] = None


@dataclass
class FetchOptions:
    iterator: t.Optional[str] = None
    limit: t.Optional[int] = None

    def to_dict(self) -> t.Dict[str, t.Any]:
        return {k: v for k, v in asdict(self).items() if v is not None}


@dataclass
class FetchOptionsMessageAttempt(FetchOptions):
    status: t.Optional[MessageStatus] = None


ApiClass = t.TypeVar("ApiClass")


class ApiBase(t.Generic[ApiClass]):
    _configuration: Configuration
    _ApiClass: t.Type[ApiClass]

    def __init__(self, configuration: Configuration) -> None:
        self._configuration = configuration

    @contextmanager
    def _api(self) -> t.Generator[ApiClass, None, None]:
        with ApiClient(self._configuration) as api_client:
            yield self._ApiClass(api_client)


class Authentication(ApiBase[AuthenticationApi]):
    _ApiClass = AuthenticationApi

    def dashboard_access(self, app_id: str) -> DashboardAccessOut:
        with self._api() as api:
            return api.get_dashboard_access_api_v1_auth_dashboard_access_app_id_post(app_id=app_id)

    def logout(self):
        with self._api() as api:
            return api.logout_api_v1_auth_logout_post()


class Application(ApiBase[ApplicationApi]):
    _ApiClass = ApplicationApi

    def list(self, options: FetchOptions = FetchOptions()) -> ListResponseApplicationOut:
        with self._api() as api:
            return api.list_applications_api_v1_app_get(**options.to_dict())

    def create(self, application_in: ApplicationIn) -> ApplicationOut:
        with self._api() as api:
            return api.create_application_api_v1_app_post(application_in=application_in)

    def get(self, app_id: str) -> ApplicationOut:
        with self._api() as api:
            return api.get_application_api_v1_app_app_id_get(app_id=app_id)

    def update(self, app_id: str, application_in: ApplicationIn) -> ApplicationOut:
        with self._api() as api:
            return api.update_application_api_v1_app_app_id_put(app_id=app_id, application_in=application_in)

    def delete(self, app_id: str) -> None:
        with self._api() as api:
            return api.delete_application_api_v1_app_app_id_delete(app_id=app_id)


class Endpoint(ApiBase[EndpointApi]):
    _ApiClass = EndpointApi

    def list(self, app_id: str, options: FetchOptions = FetchOptions()) -> ListResponseEndpointOut:
        with self._api() as api:
            return api.list_endpoints_api_v1_app_app_id_endpoint_get(app_id=app_id, **options.to_dict())

    def create(self, app_id: str, endpoint_in: EndpointIn) -> EndpointOut:
        with self._api() as api:
            return api.create_endpoint_api_v1_app_app_id_endpoint_post(app_id, endpoint_in=endpoint_in)

    def get(self, app_id: str, endpoint_id: str) -> EndpointOut:
        with self._api() as api:
            return api.get_endpoint_api_v1_app_app_id_endpoint_endpoint_id_get(app_id=app_id, endpoint_id=endpoint_id)

    def update(self, app_id: str, endpoint_id: str, endpoint_in: EndpointIn) -> EndpointOut:
        with self._api() as api:
            return api.update_endpoint_api_v1_app_app_id_endpoint_endpoint_id_put(
                app_id=app_id, endpoint_id=endpoint_id, endpoint_in=endpoint_in
            )

    def delete(self, app_id: str, endpoint_id: str) -> None:
        with self._api() as api:
            return api.delete_endpoint_api_v1_app_app_id_endpoint_endpoint_id_delete(
                app_id=app_id, endpoint_id=endpoint_id
            )

    def get_secret(self, app_id: str, endpoint_id: str) -> EndpointOut:
        with self._api() as api:
            return api.get_endpoint_secret_api_v1_app_app_id_endpoint_endpoint_id_secret_get(
                app_id=app_id, endpoint_id=endpoint_id
            )


class EventType(ApiBase[EventTypeApi]):
    _ApiClass = EventTypeApi

    def list(self, options: FetchOptions = FetchOptions()) -> ListResponseEventTypeInOut:
        with self._api() as api:
            return api.list_event_types_api_v1_event_type_get(**options.to_dict())

    def create(self, event_type_in_out: EventTypeInOut) -> EventTypeInOut:
        with self._api() as api:
            return api.create_event_type_api_v1_event_type_post(event_type_in_out=event_type_in_out)

    def delete(self, event_type_name: str) -> None:
        with self._api() as api:
            return api.delete_event_type_api_v1_event_type_event_type_name_delete(event_type_name=event_type_name)


class Message(ApiBase[MessageApi]):
    _ApiClass = MessageApi

    def list(self, app_id: str, options: FetchOptions = FetchOptions()) -> ListResponseMessageOut:
        with self._api() as api:
            return api.list_messages_api_v1_app_app_id_msg_get(app_id=app_id, **options.to_dict())

    def create(self, app_id: str, message_in: MessageIn) -> MessageOut:
        with self._api() as api:
            return api.create_message_api_v1_app_app_id_msg_post(app_id=app_id, message_in=message_in)

    def get(self, app_id: str, msg_id: str) -> MessageOut:
        with self._api() as api:
            return api.get_message_api_v1_app_app_id_msg_msg_id_get(app_id=app_id, msg_id=msg_id)


class MessageAttempt(ApiBase[MessageAttemptApi]):
    _ApiClass = MessageAttemptApi

    def list(
        self, app_id: str, msg_id: str, options: FetchOptionsMessageAttempt = FetchOptionsMessageAttempt()
    ) -> ListResponseMessageAttemptOut:
        with self._api() as api:
            return api.list_attempts_api_v1_app_app_id_msg_msg_id_attempt_get(
                app_id=app_id, msg_id=msg_id, **options.to_dict()
            )

    def get(self, app_id: str, msg_id: str, attempt_id: str) -> MessageAttemptOut:
        with self._api() as api:
            return api.get_attempt_api_v1_app_app_id_msg_msg_id_attempt_attempt_id_get(
                app_id=app_id, msg_id=msg_id, attempt_id=attempt_id
            )

    def resend(self, app_id: str, msg_id: str, endpoint_id: str) -> MessageAttemptOut:
        with self._api() as api:
            return api.resend_webhook_api_v1_app_app_id_msg_msg_id_endpoint_endpoint_id_resend_post(
                app_id=app_id, msg_id=msg_id, endpoint_id=endpoint_id
            )


class Diahook:
    _configuration: Configuration

    def __init__(self, auth_token: str, options: DiahookOptions = DiahookOptions()) -> None:
        self._configuration = Configuration(host=options._test_url, access_token=auth_token)

    @property
    def authentication(self):
        return Authentication(self._configuration)

    @property
    def application(self):
        return Application(self._configuration)

    @property
    def endpoint(self):
        return Endpoint(self._configuration)

    @property
    def event_type(self):
        return EventType(self._configuration)

    @property
    def message(self):
        return Message(self._configuration)

    @property
    def message_attempt(self):
        return MessageAttempt(self._configuration)
