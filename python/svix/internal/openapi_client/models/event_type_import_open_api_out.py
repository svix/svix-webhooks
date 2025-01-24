# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar

import attr

if TYPE_CHECKING:
    from ..models.event_type_import_open_api_out_data import EventTypeImportOpenApiOutData


T = TypeVar("T", bound="EventTypeImportOpenApiOut")


@attr.s(auto_attribs=True)
class EventTypeImportOpenApiOut:
    """
    Attributes:
        data (EventTypeImportOpenApiOutData):
    """

    data: "EventTypeImportOpenApiOutData"
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        data = self.data.to_dict()

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "data": data,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.event_type_import_open_api_out_data import EventTypeImportOpenApiOutData

        d = src_dict.copy()
        data = EventTypeImportOpenApiOutData.from_dict(d.pop("data"))

        event_type_import_open_api_out = cls(
            data=data,
        )

        event_type_import_open_api_out.additional_properties = d
        return event_type_import_open_api_out

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
