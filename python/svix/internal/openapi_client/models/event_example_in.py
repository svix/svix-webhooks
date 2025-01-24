# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

T = TypeVar("T", bound="EventExampleIn")


@attr.s(auto_attribs=True)
class EventExampleIn:
    """
    Attributes:
        event_type (str): The event type's name Example: user.signup.
        example_index (Union[Unset, int]): If the event type schema contains an array of examples, chooses which one to
            send.

            Defaults to the first example. Ignored if the schema doesn't contain an array of examples.
    """

    event_type: str
    example_index: Union[Unset, int] = 0
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        event_type = self.event_type
        example_index = self.example_index

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "eventType": event_type,
            }
        )
        if example_index is not UNSET:
            field_dict["exampleIndex"] = example_index

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        event_type = d.pop("eventType")

        example_index = d.pop("exampleIndex", UNSET)

        event_example_in = cls(
            event_type=event_type,
            example_index=example_index,
        )

        event_example_in.additional_properties = d
        return event_example_in

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
