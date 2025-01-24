# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

T = TypeVar("T", bound="RotatePollerTokenIn")


@attr.s(auto_attribs=True)
class RotatePollerTokenIn:
    """
    Attributes:
        expiry (Union[Unset, None, int]): How long the token will be valid for, in seconds. Can be up to 31,536,000
            seconds (1 year).
        old_token_expiry (Union[Unset, int]): Updates the previous token's expiration, in seconds. If set to 0, the old
            token will immediately be revoked. Must be between 0 and 86,400 seconds (1 day).

            Defaults to 300 seconds (5 minutes). Default: 300.
    """

    expiry: Union[Unset, None, int] = UNSET
    old_token_expiry: Union[Unset, int] = 300
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        expiry = self.expiry
        old_token_expiry = self.old_token_expiry

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if expiry is not UNSET:
            field_dict["expiry"] = expiry
        if old_token_expiry is not UNSET:
            field_dict["oldTokenExpiry"] = old_token_expiry

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        expiry = d.pop("expiry", UNSET)

        old_token_expiry = d.pop("oldTokenExpiry", UNSET)

        rotate_poller_token_in = cls(
            expiry=expiry,
            old_token_expiry=old_token_expiry,
        )

        rotate_poller_token_in.additional_properties = d
        return rotate_poller_token_in

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
