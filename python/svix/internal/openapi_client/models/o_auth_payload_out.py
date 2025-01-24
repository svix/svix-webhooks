# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

T = TypeVar("T", bound="OAuthPayloadOut")


@attr.s(auto_attribs=True)
class OAuthPayloadOut:
    """
    Attributes:
        access_token (Union[Unset, None, str]):
        error (Union[Unset, None, str]):
        refresh_token (Union[Unset, None, str]):
    """

    access_token: Union[Unset, None, str] = UNSET
    error: Union[Unset, None, str] = UNSET
    refresh_token: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        access_token = self.access_token
        error = self.error
        refresh_token = self.refresh_token

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if access_token is not UNSET:
            field_dict["accessToken"] = access_token
        if error is not UNSET:
            field_dict["error"] = error
        if refresh_token is not UNSET:
            field_dict["refreshToken"] = refresh_token

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        access_token = d.pop("accessToken", UNSET)

        error = d.pop("error", UNSET)

        refresh_token = d.pop("refreshToken", UNSET)

        o_auth_payload_out = cls(
            access_token=access_token,
            error=error,
            refresh_token=refresh_token,
        )

        o_auth_payload_out.additional_properties = d
        return o_auth_payload_out

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
