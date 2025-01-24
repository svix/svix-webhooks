# TODO - remove this special case when we fix the generated code for empty openapi structs
import datetime
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union

import attr
from dateutil.parser import isoparse

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.application_out_metadata import ApplicationOutMetadata


T = TypeVar("T", bound="ApplicationOut")


@attr.s(auto_attribs=True)
class ApplicationOut:
    """
    Attributes:
        created_at (datetime.datetime):
        id (str): The app's ID Example: app_1srOrx2ZWZBpBUvZwXKQmoEYga2.
        metadata (ApplicationOutMetadata):
        name (str):  Example: My first application.
        updated_at (datetime.datetime):
        rate_limit (Union[Unset, None, int]):
        uid (Union[Unset, None, str]): The app's UID Example: unique-app-identifier.
    """

    created_at: datetime.datetime
    id: str
    metadata: "ApplicationOutMetadata"
    name: str
    updated_at: datetime.datetime
    rate_limit: Union[Unset, None, int] = UNSET
    uid: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        created_at = self.created_at.isoformat()

        id = self.id
        metadata = self.metadata
        name = self.name
        updated_at = self.updated_at.isoformat()

        rate_limit = self.rate_limit
        uid = self.uid

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "createdAt": created_at,
                "id": id,
                "metadata": metadata,
                "name": name,
                "updatedAt": updated_at,
            }
        )
        if rate_limit is not UNSET:
            field_dict["rateLimit"] = rate_limit
        if uid is not UNSET:
            field_dict["uid"] = uid

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        created_at = isoparse(d.pop("createdAt"))

        id = d.pop("id")

        metadata = d.pop("metadata")

        name = d.pop("name")

        updated_at = isoparse(d.pop("updatedAt"))

        rate_limit = d.pop("rateLimit", UNSET)

        uid = d.pop("uid", UNSET)

        application_out = cls(
            created_at=created_at,
            id=id,
            metadata=metadata,
            name=name,
            updated_at=updated_at,
            rate_limit=rate_limit,
            uid=uid,
        )

        application_out.additional_properties = d
        return application_out

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
