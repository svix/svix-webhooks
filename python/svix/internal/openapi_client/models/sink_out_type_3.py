# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar

import attr

from ..models.sink_out_type_3_type import SinkOutType3Type

T = TypeVar("T", bound="SinkOutType3")


@attr.s(auto_attribs=True)
class SinkOutType3:
    """
    Attributes:
        type (SinkOutType3Type):
        url (str):
    """

    type: SinkOutType3Type
    url: str
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        type = self.type.value

        url = self.url

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "type": type,
                "url": url,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        type = SinkOutType3Type(d.pop("type"))

        url = d.pop("url")

        sink_out_type_3 = cls(
            type=type,
            url=url,
        )

        sink_out_type_3.additional_properties = d
        return sink_out_type_3

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
