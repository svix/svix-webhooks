# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.border_radius_config import BorderRadiusConfig
    from ..models.font_size_config import FontSizeConfig


T = TypeVar("T", bound="CustomThemeOverride")


@attr.s(auto_attribs=True)
class CustomThemeOverride:
    """
    Attributes:
        border_radius (Union[Unset, BorderRadiusConfig]):
        font_size (Union[Unset, FontSizeConfig]):
    """

    border_radius: Union[Unset, "BorderRadiusConfig"] = UNSET
    font_size: Union[Unset, "FontSizeConfig"] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        border_radius: Union[Unset, Dict[str, Any]] = UNSET
        if not isinstance(self.border_radius, Unset):
            border_radius = self.border_radius.to_dict()

        font_size: Union[Unset, Dict[str, Any]] = UNSET
        if not isinstance(self.font_size, Unset):
            font_size = self.font_size.to_dict()

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if border_radius is not UNSET:
            field_dict["borderRadius"] = border_radius
        if font_size is not UNSET:
            field_dict["fontSize"] = font_size

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.border_radius_config import BorderRadiusConfig
        from ..models.font_size_config import FontSizeConfig

        d = src_dict.copy()
        _border_radius = d.pop("borderRadius", UNSET)
        border_radius: Union[Unset, BorderRadiusConfig]
        if isinstance(_border_radius, Unset):
            border_radius = UNSET
        else:
            border_radius = BorderRadiusConfig.from_dict(_border_radius)

        _font_size = d.pop("fontSize", UNSET)
        font_size: Union[Unset, FontSizeConfig]
        if isinstance(_font_size, Unset):
            font_size = UNSET
        else:
            font_size = FontSizeConfig.from_dict(_font_size)

        custom_theme_override = cls(
            border_radius=border_radius,
            font_size=font_size,
        )

        custom_theme_override.additional_properties = d
        return custom_theme_override

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
