# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar

import attr

from ..models.sink_in_type_0_type import SinkInType0Type

T = TypeVar("T", bound="SinkInType0")


@attr.s(auto_attribs=True)
class SinkInType0:
    """
    Attributes:
        routing_key (str):
        type (SinkInType0Type):
        uri (str):
    """

    routing_key: str
    type: SinkInType0Type
    uri: str
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        routing_key = self.routing_key
        type = self.type.value

        uri = self.uri

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "routingKey": routing_key,
                "type": type,
                "uri": uri,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        routing_key = d.pop("routingKey")

        type = SinkInType0Type(d.pop("type"))

        uri = d.pop("uri")

        sink_in_type_0 = cls(
            routing_key=routing_key,
            type=type,
            uri=uri,
        )

        sink_in_type_0.additional_properties = d
        return sink_in_type_0

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
