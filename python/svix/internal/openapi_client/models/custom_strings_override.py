# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

T = TypeVar("T", bound="CustomStringsOverride")


@attr.s(auto_attribs=True)
class CustomStringsOverride:
    """
    Attributes:
        channels_help (Union[Unset, None, str]):
        channels_many (Union[Unset, None, str]):
        channels_one (Union[Unset, None, str]):
    """

    channels_help: Union[Unset, None, str] = UNSET
    channels_many: Union[Unset, None, str] = UNSET
    channels_one: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        channels_help = self.channels_help
        channels_many = self.channels_many
        channels_one = self.channels_one

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if channels_help is not UNSET:
            field_dict["channelsHelp"] = channels_help
        if channels_many is not UNSET:
            field_dict["channelsMany"] = channels_many
        if channels_one is not UNSET:
            field_dict["channelsOne"] = channels_one

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        channels_help = d.pop("channelsHelp", UNSET)

        channels_many = d.pop("channelsMany", UNSET)

        channels_one = d.pop("channelsOne", UNSET)

        custom_strings_override = cls(
            channels_help=channels_help,
            channels_many=channels_many,
            channels_one=channels_one,
        )

        custom_strings_override.additional_properties = d
        return custom_strings_override

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
