# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

T = TypeVar("T", bound="ApplicationTokenExpireIn")


@attr.s(auto_attribs=True)
class ApplicationTokenExpireIn:
    """
    Attributes:
        expiry (Union[Unset, None, int]): How many seconds until the old key is expired. Example: 60.
    """

    expiry: Union[Unset, None, int] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        expiry = self.expiry

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if expiry is not UNSET:
            field_dict["expiry"] = expiry

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        expiry = d.pop("expiry", UNSET)

        application_token_expire_in = cls(
            expiry=expiry,
        )

        application_token_expire_in.additional_properties = d
        return application_token_expire_in

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
