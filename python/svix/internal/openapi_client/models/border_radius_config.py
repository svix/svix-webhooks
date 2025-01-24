# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..models.border_radius_enum import BorderRadiusEnum
from ..types import UNSET, Unset

T = TypeVar("T", bound="BorderRadiusConfig")


@attr.s(auto_attribs=True)
class BorderRadiusConfig:
    """
    Attributes:
        button (Union[Unset, BorderRadiusEnum]):
        card (Union[Unset, BorderRadiusEnum]):
        input_ (Union[Unset, BorderRadiusEnum]):
    """

    button: Union[Unset, BorderRadiusEnum] = UNSET
    card: Union[Unset, BorderRadiusEnum] = UNSET
    input_: Union[Unset, BorderRadiusEnum] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        button: Union[Unset, str] = UNSET
        if not isinstance(self.button, Unset):
            button = self.button.value

        card: Union[Unset, str] = UNSET
        if not isinstance(self.card, Unset):
            card = self.card.value

        input_: Union[Unset, str] = UNSET
        if not isinstance(self.input_, Unset):
            input_ = self.input_.value

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if button is not UNSET:
            field_dict["button"] = button
        if card is not UNSET:
            field_dict["card"] = card
        if input_ is not UNSET:
            field_dict["input"] = input_

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        _button = d.pop("button", UNSET)
        button: Union[Unset, BorderRadiusEnum]
        if isinstance(_button, Unset):
            button = UNSET
        else:
            button = BorderRadiusEnum(_button)

        _card = d.pop("card", UNSET)
        card: Union[Unset, BorderRadiusEnum]
        if isinstance(_card, Unset):
            card = UNSET
        else:
            card = BorderRadiusEnum(_card)

        _input_ = d.pop("input", UNSET)
        input_: Union[Unset, BorderRadiusEnum]
        if isinstance(_input_, Unset):
            input_ = UNSET
        else:
            input_ = BorderRadiusEnum(_input_)

        border_radius_config = cls(
            button=button,
            card=card,
            input_=input_,
        )

        border_radius_config.additional_properties = d
        return border_radius_config

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
