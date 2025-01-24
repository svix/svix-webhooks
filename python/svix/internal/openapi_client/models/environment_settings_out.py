# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.custom_color_palette import CustomColorPalette
    from ..models.custom_strings_override import CustomStringsOverride
    from ..models.custom_theme_override import CustomThemeOverride


T = TypeVar("T", bound="EnvironmentSettingsOut")


@attr.s(auto_attribs=True)
class EnvironmentSettingsOut:
    """
    Attributes:
        color_palette_dark (Union[Unset, CustomColorPalette]):
        color_palette_light (Union[Unset, CustomColorPalette]):
        custom_color (Union[Unset, None, str]):
        custom_font_family (Union[Unset, None, str]):  Example: Open Sans.
        custom_font_family_url (Union[Unset, None, str]):
        custom_logo_url (Union[Unset, None, str]):
        custom_strings_override (Union[Unset, CustomStringsOverride]):
        custom_theme_override (Union[Unset, CustomThemeOverride]):
        display_name (Union[Unset, None, str]):
        enable_channels (Union[Unset, bool]):
        enable_integration_management (Union[Unset, bool]):
        enable_message_stream (Union[Unset, bool]):
        enable_message_tags (Union[Unset, bool]):
        enable_transformations (Union[Unset, bool]):
        show_use_svix_play (Union[Unset, bool]):  Default: True.
        wipe_successful_payload (Union[Unset, bool]):
    """

    color_palette_dark: Union[Unset, "CustomColorPalette"] = UNSET
    color_palette_light: Union[Unset, "CustomColorPalette"] = UNSET
    custom_color: Union[Unset, None, str] = UNSET
    custom_font_family: Union[Unset, None, str] = UNSET
    custom_font_family_url: Union[Unset, None, str] = UNSET
    custom_logo_url: Union[Unset, None, str] = UNSET
    custom_strings_override: Union[Unset, "CustomStringsOverride"] = UNSET
    custom_theme_override: Union[Unset, "CustomThemeOverride"] = UNSET
    display_name: Union[Unset, None, str] = UNSET
    enable_channels: Union[Unset, bool] = False
    enable_integration_management: Union[Unset, bool] = False
    enable_message_stream: Union[Unset, bool] = False
    enable_message_tags: Union[Unset, bool] = False
    enable_transformations: Union[Unset, bool] = False
    show_use_svix_play: Union[Unset, bool] = True
    wipe_successful_payload: Union[Unset, bool] = False
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        color_palette_dark: Union[Unset, Dict[str, Any]] = UNSET
        if not isinstance(self.color_palette_dark, Unset):
            color_palette_dark = self.color_palette_dark.to_dict()

        color_palette_light: Union[Unset, Dict[str, Any]] = UNSET
        if not isinstance(self.color_palette_light, Unset):
            color_palette_light = self.color_palette_light.to_dict()

        custom_color = self.custom_color
        custom_font_family = self.custom_font_family
        custom_font_family_url = self.custom_font_family_url
        custom_logo_url = self.custom_logo_url
        custom_strings_override: Union[Unset, Dict[str, Any]] = UNSET
        if not isinstance(self.custom_strings_override, Unset):
            custom_strings_override = self.custom_strings_override.to_dict()

        custom_theme_override: Union[Unset, Dict[str, Any]] = UNSET
        if not isinstance(self.custom_theme_override, Unset):
            custom_theme_override = self.custom_theme_override.to_dict()

        display_name = self.display_name
        enable_channels = self.enable_channels
        enable_integration_management = self.enable_integration_management
        enable_message_stream = self.enable_message_stream
        enable_message_tags = self.enable_message_tags
        enable_transformations = self.enable_transformations
        show_use_svix_play = self.show_use_svix_play
        wipe_successful_payload = self.wipe_successful_payload

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if color_palette_dark is not UNSET:
            field_dict["colorPaletteDark"] = color_palette_dark
        if color_palette_light is not UNSET:
            field_dict["colorPaletteLight"] = color_palette_light
        if custom_color is not UNSET:
            field_dict["customColor"] = custom_color
        if custom_font_family is not UNSET:
            field_dict["customFontFamily"] = custom_font_family
        if custom_font_family_url is not UNSET:
            field_dict["customFontFamilyUrl"] = custom_font_family_url
        if custom_logo_url is not UNSET:
            field_dict["customLogoUrl"] = custom_logo_url
        if custom_strings_override is not UNSET:
            field_dict["customStringsOverride"] = custom_strings_override
        if custom_theme_override is not UNSET:
            field_dict["customThemeOverride"] = custom_theme_override
        if display_name is not UNSET:
            field_dict["displayName"] = display_name
        if enable_channels is not UNSET:
            field_dict["enableChannels"] = enable_channels
        if enable_integration_management is not UNSET:
            field_dict["enableIntegrationManagement"] = enable_integration_management
        if enable_message_stream is not UNSET:
            field_dict["enableMessageStream"] = enable_message_stream
        if enable_message_tags is not UNSET:
            field_dict["enableMessageTags"] = enable_message_tags
        if enable_transformations is not UNSET:
            field_dict["enableTransformations"] = enable_transformations
        if show_use_svix_play is not UNSET:
            field_dict["showUseSvixPlay"] = show_use_svix_play
        if wipe_successful_payload is not UNSET:
            field_dict["wipeSuccessfulPayload"] = wipe_successful_payload

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.custom_color_palette import CustomColorPalette
        from ..models.custom_strings_override import CustomStringsOverride
        from ..models.custom_theme_override import CustomThemeOverride

        d = src_dict.copy()
        _color_palette_dark = d.pop("colorPaletteDark", UNSET)
        color_palette_dark: Union[Unset, CustomColorPalette]
        if isinstance(_color_palette_dark, Unset):
            color_palette_dark = UNSET
        else:
            color_palette_dark = CustomColorPalette.from_dict(_color_palette_dark)

        _color_palette_light = d.pop("colorPaletteLight", UNSET)
        color_palette_light: Union[Unset, CustomColorPalette]
        if isinstance(_color_palette_light, Unset):
            color_palette_light = UNSET
        else:
            color_palette_light = CustomColorPalette.from_dict(_color_palette_light)

        custom_color = d.pop("customColor", UNSET)

        custom_font_family = d.pop("customFontFamily", UNSET)

        custom_font_family_url = d.pop("customFontFamilyUrl", UNSET)

        custom_logo_url = d.pop("customLogoUrl", UNSET)

        _custom_strings_override = d.pop("customStringsOverride", UNSET)
        custom_strings_override: Union[Unset, CustomStringsOverride]
        if isinstance(_custom_strings_override, Unset):
            custom_strings_override = UNSET
        else:
            custom_strings_override = CustomStringsOverride.from_dict(_custom_strings_override)

        _custom_theme_override = d.pop("customThemeOverride", UNSET)
        custom_theme_override: Union[Unset, CustomThemeOverride]
        if isinstance(_custom_theme_override, Unset):
            custom_theme_override = UNSET
        else:
            custom_theme_override = CustomThemeOverride.from_dict(_custom_theme_override)

        display_name = d.pop("displayName", UNSET)

        enable_channels = d.pop("enableChannels", UNSET)

        enable_integration_management = d.pop("enableIntegrationManagement", UNSET)

        enable_message_stream = d.pop("enableMessageStream", UNSET)

        enable_message_tags = d.pop("enableMessageTags", UNSET)

        enable_transformations = d.pop("enableTransformations", UNSET)

        show_use_svix_play = d.pop("showUseSvixPlay", UNSET)

        wipe_successful_payload = d.pop("wipeSuccessfulPayload", UNSET)

        environment_settings_out = cls(
            color_palette_dark=color_palette_dark,
            color_palette_light=color_palette_light,
            custom_color=custom_color,
            custom_font_family=custom_font_family,
            custom_font_family_url=custom_font_family_url,
            custom_logo_url=custom_logo_url,
            custom_strings_override=custom_strings_override,
            custom_theme_override=custom_theme_override,
            display_name=display_name,
            enable_channels=enable_channels,
            enable_integration_management=enable_integration_management,
            enable_message_stream=enable_message_stream,
            enable_message_tags=enable_message_tags,
            enable_transformations=enable_transformations,
            show_use_svix_play=show_use_svix_play,
            wipe_successful_payload=wipe_successful_payload,
        )

        environment_settings_out.additional_properties = d
        return environment_settings_out

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
