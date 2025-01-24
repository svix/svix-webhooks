# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar

import attr

from ..models.endpoint_updated_event_type import EndpointUpdatedEventType

if TYPE_CHECKING:
    from ..models.endpoint_updated_event_data import EndpointUpdatedEventData


T = TypeVar("T", bound="EndpointUpdatedEvent")


@attr.s(auto_attribs=True)
class EndpointUpdatedEvent:
    """Sent when an endpoint is updated.

    Attributes:
        data (EndpointUpdatedEventData): Sent when an endpoint is created, updated, or deleted
        type (EndpointUpdatedEventType):  Default: EndpointUpdatedEventType.ENDPOINT_UPDATED.
    """

    data: "EndpointUpdatedEventData"
    type: EndpointUpdatedEventType = EndpointUpdatedEventType.ENDPOINT_UPDATED
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
        from ..models.endpoint_updated_event_data import EndpointUpdatedEventData

        d = src_dict.copy()
        data = EndpointUpdatedEventData.from_dict(d.pop("data"))

        type = EndpointUpdatedEventType(d.pop("type"))

        endpoint_updated_event = cls(
            data=data,
            type=type,
        )

        endpoint_updated_event.additional_properties = d
        return endpoint_updated_event

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
