# TODO - remove this special case when we fix the generated code for empty openapi structs
import datetime
from typing import Any, Dict, List, Type, TypeVar

import attr
from dateutil.parser import isoparse

T = TypeVar("T", bound="IntegrationOut")


@attr.s(auto_attribs=True)
class IntegrationOut:
    """
    Attributes:
        created_at (datetime.datetime):
        id (str): The integ's ID Example: integ_1srOrx2ZWZBpBUvZwXKQmoEYga2.
        name (str):  Example: Example Integration.
        updated_at (datetime.datetime):
    """

    created_at: datetime.datetime
    id: str
    name: str
    updated_at: datetime.datetime
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        created_at = self.created_at.isoformat()

        id = self.id
        name = self.name
        updated_at = self.updated_at.isoformat()

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "createdAt": created_at,
                "id": id,
                "name": name,
                "updatedAt": updated_at,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        created_at = isoparse(d.pop("createdAt"))

        id = d.pop("id")

        name = d.pop("name")

        updated_at = isoparse(d.pop("updatedAt"))

        integration_out = cls(
            created_at=created_at,
            id=id,
            name=name,
            updated_at=updated_at,
        )

        integration_out.additional_properties = d
        return integration_out

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
