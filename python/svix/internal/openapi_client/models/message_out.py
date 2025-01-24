# TODO - remove this special case when we fix the generated code for empty openapi structs
import datetime
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union, cast

import attr
from dateutil.parser import isoparse

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.message_out_payload import MessageOutPayload


T = TypeVar("T", bound="MessageOut")


@attr.s(auto_attribs=True)
class MessageOut:
    """
    Attributes:
        event_type (str): The event type's name Example: user.signup.
        id (str): The msg's ID Example: msg_1srOrx2ZWZBpBUvZwXKQmoEYga2.
        payload (MessageOutPayload):  Example: {'email': 'test@example.com', 'type': 'user.created', 'username':
            'test_user'}.
        timestamp (datetime.datetime):
        channels (Union[Unset, None, List[str]]): List of free-form identifiers that endpoints can filter by Example:
            ['project_123', 'group_2'].
        event_id (Union[Unset, None, str]): Optional unique identifier for the message Example: unique-msg-identifier.
        tags (Union[Unset, None, List[str]]):
    """

    event_type: str
    id: str
    payload: "MessageOutPayload"
    timestamp: datetime.datetime
    channels: Union[Unset, None, List[str]] = UNSET
    event_id: Union[Unset, None, str] = UNSET
    tags: Union[Unset, None, List[str]] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        event_type = self.event_type
        id = self.id
        payload = self.payload
        timestamp = self.timestamp.isoformat()

        channels: Union[Unset, None, List[str]] = UNSET
        if not isinstance(self.channels, Unset):
            if self.channels is None:
                channels = None
            else:
                channels = self.channels

        event_id = self.event_id
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
                "timestamp": timestamp,
            }
        )
        if channels is not UNSET:
            field_dict["channels"] = channels
        if event_id is not UNSET:
            field_dict["eventId"] = event_id
        if tags is not UNSET:
            field_dict["tags"] = tags

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        event_type = d.pop("eventType")

        id = d.pop("id")

        payload = d.pop("payload")

        timestamp = isoparse(d.pop("timestamp"))

        channels = cast(List[str], d.pop("channels", UNSET))

        event_id = d.pop("eventId", UNSET)

        tags = cast(List[str], d.pop("tags", UNSET))

        message_out = cls(
            event_type=event_type,
            id=id,
            payload=payload,
            timestamp=timestamp,
            channels=channels,
            event_id=event_id,
            tags=tags,
        )

        message_out.additional_properties = d
        return message_out

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
