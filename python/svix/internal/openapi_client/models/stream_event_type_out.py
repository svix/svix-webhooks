# TODO - remove this special case when we fix the generated code for empty openapi structs
import datetime
from typing import Any, Dict, List, Type, TypeVar, Union

import attr
from dateutil.parser import isoparse

from ..types import UNSET, Unset

T = TypeVar("T", bound="StreamEventTypeOut")


@attr.s(auto_attribs=True)
class StreamEventTypeOut:
    """
    Attributes:
        created_at (datetime.datetime):
        name (str): The event type's name Example: user.signup.
        updated_at (datetime.datetime):
        description (Union[Unset, None, str]):
    """

    created_at: datetime.datetime
    name: str
    updated_at: datetime.datetime
    description: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        created_at = self.created_at.isoformat()

        name = self.name
        updated_at = self.updated_at.isoformat()

        description = self.description

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "createdAt": created_at,
                "name": name,
                "updatedAt": updated_at,
            }
        )
        if description is not UNSET:
            field_dict["description"] = description

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        created_at = isoparse(d.pop("createdAt"))

        name = d.pop("name")

        updated_at = isoparse(d.pop("updatedAt"))

        description = d.pop("description", UNSET)

        stream_event_type_out = cls(
            created_at=created_at,
            name=name,
            updated_at=updated_at,
            description=description,
        )

        stream_event_type_out.additional_properties = d
        return stream_event_type_out

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
