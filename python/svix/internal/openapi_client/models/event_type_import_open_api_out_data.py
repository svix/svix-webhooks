# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union, cast

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.event_type_from_open_api import EventTypeFromOpenApi


T = TypeVar("T", bound="EventTypeImportOpenApiOutData")


@attr.s(auto_attribs=True)
class EventTypeImportOpenApiOutData:
    """
    Attributes:
        modified (List[str]):
        to_modify (Union[Unset, None, List['EventTypeFromOpenApi']]):
    """

    modified: List[str]
    to_modify: Union[Unset, None, List["EventTypeFromOpenApi"]] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        modified = self.modified

        to_modify: Union[Unset, None, List[Dict[str, Any]]] = UNSET
        if not isinstance(self.to_modify, Unset):
            if self.to_modify is None:
                to_modify = None
            else:
                to_modify = []
                for to_modify_item_data in self.to_modify:
                    to_modify_item = to_modify_item_data.to_dict()

                    to_modify.append(to_modify_item)

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "modified": modified,
            }
        )
        if to_modify is not UNSET:
            field_dict["to_modify"] = to_modify

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.event_type_from_open_api import EventTypeFromOpenApi

        d = src_dict.copy()
        modified = cast(List[str], d.pop("modified"))

        to_modify = []
        _to_modify = d.pop("to_modify", UNSET)
        for to_modify_item_data in _to_modify or []:
            to_modify_item = EventTypeFromOpenApi.from_dict(to_modify_item_data)

            to_modify.append(to_modify_item)

        event_type_import_open_api_out_data = cls(
            modified=modified,
            to_modify=to_modify,
        )

        event_type_import_open_api_out_data.additional_properties = d
        return event_type_import_open_api_out_data

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
