# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.application_patch_metadata import ApplicationPatchMetadata


T = TypeVar("T", bound="ApplicationPatch")


@attr.s(auto_attribs=True)
class ApplicationPatch:
    """
    Attributes:
        metadata (Union[Unset, ApplicationPatchMetadata]):
        name (Union[Unset, str]):
        rate_limit (Union[Unset, None, int]):
        uid (Union[Unset, None, str]): The app's UID Example: unique-app-identifier.
    """

    metadata: Union[Unset, "ApplicationPatchMetadata"] = UNSET
    name: Union[Unset, str] = UNSET
    rate_limit: Union[Unset, None, int] = UNSET
    uid: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        metadata = self.metadata
        name = self.name
        rate_limit = self.rate_limit
        uid = self.uid

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if metadata is not UNSET:
            field_dict["metadata"] = metadata
        if name is not UNSET:
            field_dict["name"] = name
        if rate_limit is not UNSET:
            field_dict["rateLimit"] = rate_limit
        if uid is not UNSET:
            field_dict["uid"] = uid

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        metadata = d.pop("metadata", UNSET)

        name = d.pop("name", UNSET)

        rate_limit = d.pop("rateLimit", UNSET)

        uid = d.pop("uid", UNSET)

        application_patch = cls(
            metadata=metadata,
            name=name,
            rate_limit=rate_limit,
            uid=uid,
        )

        application_patch.additional_properties = d
        return application_patch

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
