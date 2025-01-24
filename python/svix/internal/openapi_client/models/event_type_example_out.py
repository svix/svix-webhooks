# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar

import attr

if TYPE_CHECKING:
    from ..models.event_type_example_out_example import EventTypeExampleOutExample


T = TypeVar("T", bound="EventTypeExampleOut")


@attr.s(auto_attribs=True)
class EventTypeExampleOut:
    """
    Attributes:
        example (EventTypeExampleOutExample):  Example: {'data': {'email': 'test@example.com', 'username': 'test_user'},
            'type': 'user.created'}.
    """

    example: "EventTypeExampleOutExample"
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        example = self.example

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "example": example,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        example = d.pop("example")

        event_type_example_out = cls(
            example=example,
        )

        event_type_example_out.additional_properties = d
        return event_type_example_out

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
