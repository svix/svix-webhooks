# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Optional, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

T = TypeVar("T", bound="StreamEventTypePatch")


@attr.s(auto_attribs=True)
class StreamEventTypePatch:
    """
    Attributes:
        description (Optional[str]):
        name (Union[Unset, None, str]): The event type's name Example: user.signup.
    """

    description: Optional[str]
    name: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        description = self.description
        name = self.name

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "description": description,
            }
        )
        if name is not UNSET:
            field_dict["name"] = name

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        description = d.pop("description")

        name = d.pop("name", UNSET)

        stream_event_type_patch = cls(
            description=description,
            name=name,
        )

        stream_event_type_patch.additional_properties = d
        return stream_event_type_patch

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
