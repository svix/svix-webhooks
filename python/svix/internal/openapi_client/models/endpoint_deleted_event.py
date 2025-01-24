# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar

import attr

from ..models.endpoint_deleted_event_type import EndpointDeletedEventType

if TYPE_CHECKING:
    from ..models.endpoint_deleted_event_data import EndpointDeletedEventData


T = TypeVar("T", bound="EndpointDeletedEvent")


@attr.s(auto_attribs=True)
class EndpointDeletedEvent:
    """Sent when an endpoint is deleted.

    Attributes:
        data (EndpointDeletedEventData): Sent when an endpoint is created, updated, or deleted
        type (EndpointDeletedEventType):  Default: EndpointDeletedEventType.ENDPOINT_DELETED.
    """

    data: "EndpointDeletedEventData"
    type: EndpointDeletedEventType = EndpointDeletedEventType.ENDPOINT_DELETED
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
        from ..models.endpoint_deleted_event_data import EndpointDeletedEventData

        d = src_dict.copy()
        data = EndpointDeletedEventData.from_dict(d.pop("data"))

        type = EndpointDeletedEventType(d.pop("type"))

        endpoint_deleted_event = cls(
            data=data,
            type=type,
        )

        endpoint_deleted_event.additional_properties = d
        return endpoint_deleted_event

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
