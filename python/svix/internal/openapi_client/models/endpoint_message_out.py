# TODO - remove this special case when we fix the generated code for empty openapi structs
import datetime
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union, cast

import attr
from dateutil.parser import isoparse

from ..models.message_status import MessageStatus
from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.endpoint_message_out_payload import EndpointMessageOutPayload


T = TypeVar("T", bound="EndpointMessageOut")


@attr.s(auto_attribs=True)
class EndpointMessageOut:
    """A model containing information on a given message plus additional fields on the last attempt for that message.

    Attributes:
        event_type (str): The event type's name Example: user.signup.
        id (str): The msg's ID Example: msg_1srOrx2ZWZBpBUvZwXKQmoEYga2.
        payload (EndpointMessageOutPayload):  Example: {'email': 'test@example.com', 'type': 'user.created', 'username':
            'test_user'}.
        status (MessageStatus): The sending status of the message:
            - Success = 0
            - Pending = 1
            - Fail = 2
            - Sending = 3
        timestamp (datetime.datetime):
        channels (Union[Unset, None, List[str]]): List of free-form identifiers that endpoints can filter by Example:
            ['project_123', 'group_2'].
        event_id (Union[Unset, None, str]): Optional unique identifier for the message Example: unique-msg-identifier.
        next_attempt (Union[Unset, None, datetime.datetime]):
        tags (Union[Unset, None, List[str]]):
    """

    event_type: str
    id: str
    payload: "EndpointMessageOutPayload"
    status: MessageStatus
    timestamp: datetime.datetime
    channels: Union[Unset, None, List[str]] = UNSET
    event_id: Union[Unset, None, str] = UNSET
    next_attempt: Union[Unset, None, datetime.datetime] = UNSET
    tags: Union[Unset, None, List[str]] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        event_type = self.event_type
        id = self.id
        payload = self.payload
        status = self.status.value

        timestamp = self.timestamp.isoformat()

        channels: Union[Unset, None, List[str]] = UNSET
        if not isinstance(self.channels, Unset):
            if self.channels is None:
                channels = None
            else:
                channels = self.channels

        event_id = self.event_id
        next_attempt: Union[Unset, None, str] = UNSET
        if not isinstance(self.next_attempt, Unset):
            next_attempt = self.next_attempt.isoformat() if self.next_attempt else None

        tags: Union[Unset, None, List[str]] = UNSET
        if not isinstance(self.tags, Unset):
            if self.tags is None:
                tags = None
            else:
                tags = self.tags

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "eventType": event_type,
                "id": id,
                "payload": payload,
                "status": status,
                "timestamp": timestamp,
            }
        )
        if channels is not UNSET:
            field_dict["channels"] = channels
        if event_id is not UNSET:
            field_dict["eventId"] = event_id
        if next_attempt is not UNSET:
            field_dict["nextAttempt"] = next_attempt
        if tags is not UNSET:
            field_dict["tags"] = tags

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        event_type = d.pop("eventType")

        id = d.pop("id")

        payload = d.pop("payload")

        status = MessageStatus(d.pop("status"))

        timestamp = isoparse(d.pop("timestamp"))

        channels = cast(List[str], d.pop("channels", UNSET))

        event_id = d.pop("eventId", UNSET)

        _next_attempt = d.pop("nextAttempt", UNSET)
        next_attempt: Union[Unset, None, datetime.datetime]
        if _next_attempt is None:
            next_attempt = None
        elif isinstance(_next_attempt, Unset):
            next_attempt = UNSET
        else:
            next_attempt = isoparse(_next_attempt)

        tags = cast(List[str], d.pop("tags", UNSET))

        endpoint_message_out = cls(
            event_type=event_type,
            id=id,
            payload=payload,
            status=status,
            timestamp=timestamp,
            channels=channels,
            event_id=event_id,
            next_attempt=next_attempt,
            tags=tags,
        )

        endpoint_message_out.additional_properties = d
        return endpoint_message_out

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
