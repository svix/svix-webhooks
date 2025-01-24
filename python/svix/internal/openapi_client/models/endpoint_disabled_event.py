# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar

import attr

from ..models.endpoint_disabled_event_type import EndpointDisabledEventType

if TYPE_CHECKING:
    from ..models.endpoint_disabled_event_data import EndpointDisabledEventData


T = TypeVar("T", bound="EndpointDisabledEvent")


@attr.s(auto_attribs=True)
class EndpointDisabledEvent:
    """Sent when an endpoint has been automatically disabled after continuous failures.

    Attributes:
        data (EndpointDisabledEventData): Sent when an endpoint has been automatically disabled after continuous
            failures.
        type (EndpointDisabledEventType):  Default: EndpointDisabledEventType.ENDPOINT_DISABLED.
    """

    data: "EndpointDisabledEventData"
    type: EndpointDisabledEventType = EndpointDisabledEventType.ENDPOINT_DISABLED
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        data = self.data.to_dict()

        type = self.type.value

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "data": data,
                "type": type,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.endpoint_disabled_event_data import EndpointDisabledEventData

        d = src_dict.copy()
        data = EndpointDisabledEventData.from_dict(d.pop("data"))

        type = EndpointDisabledEventType(d.pop("type"))

        endpoint_disabled_event = cls(
            data=data,
            type=type,
        )

        endpoint_disabled_event.additional_properties = d
        return endpoint_disabled_event

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
