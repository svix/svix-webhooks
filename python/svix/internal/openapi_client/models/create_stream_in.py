# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.event_in import EventIn
    from ..models.stream_in import StreamIn


T = TypeVar("T", bound="CreateStreamIn")


@attr.s(auto_attribs=True)
class CreateStreamIn:
    """
    Attributes:
        messages (List['EventIn']):
        stream (Union[Unset, StreamIn]):
    """

    messages: List["EventIn"]
    stream: Union[Unset, "StreamIn"] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        messages = []
        for messages_item_data in self.messages:
            messages_item = messages_item_data.to_dict()

            messages.append(messages_item)

        stream: Union[Unset, Dict[str, Any]] = UNSET
        if not isinstance(self.stream, Unset):
            stream = self.stream.to_dict()

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "messages": messages,
            }
        )
        if stream is not UNSET:
            field_dict["stream"] = stream

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.event_in import EventIn
        from ..models.stream_in import StreamIn

        d = src_dict.copy()
        messages = []
        _messages = d.pop("messages")
        for messages_item_data in _messages:
            messages_item = EventIn.from_dict(messages_item_data)

            messages.append(messages_item)

        _stream = d.pop("stream", UNSET)
        stream: Union[Unset, StreamIn]
        if isinstance(_stream, Unset):
            stream = UNSET
        else:
            stream = StreamIn.from_dict(_stream)

        create_stream_in = cls(
            messages=messages,
            stream=stream,
        )

        create_stream_in.additional_properties = d
        return create_stream_in

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
