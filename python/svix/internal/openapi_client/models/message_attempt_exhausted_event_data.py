# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.message_attempt_failed_data import MessageAttemptFailedData


T = TypeVar("T", bound="MessageAttemptExhaustedEventData")


@attr.s(auto_attribs=True)
class MessageAttemptExhaustedEventData:
    """Sent when a message delivery has failed (all of the retry attempts have been exhausted) as a
    "message.attempt.exhausted" type or after it's failed four times as a "message.attempt.failing" event.

        Attributes:
            app_id (str): The app's ID Example: app_1srOrx2ZWZBpBUvZwXKQmoEYga2.
            endpoint_id (str): The ep's ID Example: ep_1srOrx2ZWZBpBUvZwXKQmoEYga2.
            last_attempt (MessageAttemptFailedData):
            msg_id (str): The msg's ID Example: msg_1srOrx2ZWZBpBUvZwXKQmoEYga2.
            app_uid (Union[Unset, None, str]): The app's UID Example: unique-app-identifier.
            msg_event_id (Union[Unset, None, str]): The msg's UID Example: unique-msg-identifier.
    """

    app_id: str
    endpoint_id: str
    last_attempt: "MessageAttemptFailedData"
    msg_id: str
    app_uid: Union[Unset, None, str] = UNSET
    msg_event_id: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        app_id = self.app_id
        endpoint_id = self.endpoint_id
        last_attempt = self.last_attempt.to_dict()

        msg_id = self.msg_id
        app_uid = self.app_uid
        msg_event_id = self.msg_event_id

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "appId": app_id,
                "endpointId": endpoint_id,
                "lastAttempt": last_attempt,
                "msgId": msg_id,
            }
        )
        if app_uid is not UNSET:
            field_dict["appUid"] = app_uid
        if msg_event_id is not UNSET:
            field_dict["msgEventId"] = msg_event_id

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.message_attempt_failed_data import MessageAttemptFailedData

        d = src_dict.copy()
        app_id = d.pop("appId")

        endpoint_id = d.pop("endpointId")

        last_attempt = MessageAttemptFailedData.from_dict(d.pop("lastAttempt"))

        msg_id = d.pop("msgId")

        app_uid = d.pop("appUid", UNSET)

        msg_event_id = d.pop("msgEventId", UNSET)

        message_attempt_exhausted_event_data = cls(
            app_id=app_id,
            endpoint_id=endpoint_id,
            last_attempt=last_attempt,
            msg_id=msg_id,
            app_uid=app_uid,
            msg_event_id=msg_event_id,
        )

        message_attempt_exhausted_event_data.additional_properties = d
        return message_attempt_exhausted_event_data

    @property
    def additional_keys(self) -> List[str]:
        return list(self.additional_properties.keys())

    def __getitem__(self, key: str) -> Any:
        return self.additional_properties[key]

    def __setitem__(self, key: str, value: Any) -> None:
        self.additional_properties[key] = value

    def __delitem__(self, key: str) -> None:
        del self.additional_properties[key]

    def __contains__(self, key: str) -> bool:
        return key in self.additional_properties
