# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.application_in_metadata import ApplicationInMetadata


T = TypeVar("T", bound="ApplicationIn")


@attr.s(auto_attribs=True)
class ApplicationIn:
    """
    Attributes:
        name (str):  Example: My first application.
        metadata (Union[Unset, ApplicationInMetadata]):
        rate_limit (Union[Unset, None, int]):
        uid (Union[Unset, None, str]): Optional unique identifier for the application. Example: unique-app-identifier.
    """

    name: str
    metadata: Union[Unset, "ApplicationInMetadata"] = UNSET
    rate_limit: Union[Unset, None, int] = UNSET
    uid: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        name = self.name
        metadata = self.metadata
        rate_limit = self.rate_limit
        uid = self.uid

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "name": name,
            }
        )
        if metadata is not UNSET:
            field_dict["metadata"] = metadata
        if rate_limit is not UNSET:
            field_dict["rateLimit"] = rate_limit
        if uid is not UNSET:
            field_dict["uid"] = uid

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        name = d.pop("name")

        metadata = d.pop("metadata", UNSET)

        rate_limit = d.pop("rateLimit", UNSET)

        uid = d.pop("uid", UNSET)

        application_in = cls(
            name=name,
            metadata=metadata,
            rate_limit=rate_limit,
            uid=uid,
        )

        application_in.additional_properties = d
        return application_in

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
