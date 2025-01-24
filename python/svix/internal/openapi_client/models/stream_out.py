# TODO - remove this special case when we fix the generated code for empty openapi structs
import datetime
from typing import Any, Dict, List, Type, TypeVar, Union

import attr
from dateutil.parser import isoparse

from ..types import UNSET, Unset

T = TypeVar("T", bound="StreamOut")


@attr.s(auto_attribs=True)
class StreamOut:
    """
    Attributes:
        created_at (datetime.datetime):
        id (str):
        updated_at (datetime.datetime):
        description (Union[Unset, None, str]):
        uid (Union[Unset, str]):
    """

    created_at: datetime.datetime
    id: str
    updated_at: datetime.datetime
    description: Union[Unset, None, str] = UNSET
    uid: Union[Unset, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        created_at = self.created_at.isoformat()

        id = self.id
        updated_at = self.updated_at.isoformat()

        description = self.description
        uid = self.uid

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "createdAt": created_at,
                "id": id,
                "updatedAt": updated_at,
            }
        )
        if description is not UNSET:
            field_dict["description"] = description
        if uid is not UNSET:
            field_dict["uid"] = uid

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        created_at = isoparse(d.pop("createdAt"))

        id = d.pop("id")

        updated_at = isoparse(d.pop("updatedAt"))

        description = d.pop("description", UNSET)

        uid = d.pop("uid", UNSET)

        stream_out = cls(
            created_at=created_at,
            id=id,
            updated_at=updated_at,
            description=description,
            uid=uid,
        )

        stream_out.additional_properties = d
        return stream_out

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
