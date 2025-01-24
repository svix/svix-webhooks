# TODO - remove this special case when we fix the generated code for empty openapi structs
import datetime
from typing import TYPE_CHECKING, Any, Dict, List, Optional, Type, TypeVar, Union

import attr
from dateutil.parser import isoparse

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.environment_out_settings import EnvironmentOutSettings
    from ..models.event_type_out import EventTypeOut
    from ..models.template_out import TemplateOut


T = TypeVar("T", bound="EnvironmentOut")


@attr.s(auto_attribs=True)
class EnvironmentOut:
    """
    Attributes:
        created_at (datetime.datetime):
        event_types (List['EventTypeOut']):
        transformation_templates (List['TemplateOut']):
        settings (Optional[EnvironmentOutSettings]):
        version (Union[Unset, int]):  Default: 1.
    """

    created_at: datetime.datetime
    event_types: List["EventTypeOut"]
    transformation_templates: List["TemplateOut"]
    settings: Optional["EnvironmentOutSettings"]
    version: Union[Unset, int] = 1
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        created_at = self.created_at.isoformat()

        event_types = []
        for event_types_item_data in self.event_types:
            event_types_item = event_types_item_data.to_dict()

            event_types.append(event_types_item)

        transformation_templates = []
        for transformation_templates_item_data in self.transformation_templates:
            transformation_templates_item = transformation_templates_item_data.to_dict()

            transformation_templates.append(transformation_templates_item)

        settings = self.settings
        version = self.version

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "createdAt": created_at,
                "eventTypes": event_types,
                "transformationTemplates": transformation_templates,
                "settings": settings,
            }
        )
        if version is not UNSET:
            field_dict["version"] = version

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.event_type_out import EventTypeOut
        from ..models.template_out import TemplateOut

        d = src_dict.copy()
        created_at = isoparse(d.pop("createdAt"))

        event_types = []
        _event_types = d.pop("eventTypes")
        for event_types_item_data in _event_types:
            event_types_item = EventTypeOut.from_dict(event_types_item_data)

            event_types.append(event_types_item)

        transformation_templates = []
        _transformation_templates = d.pop("transformationTemplates")
        for transformation_templates_item_data in _transformation_templates:
            transformation_templates_item = TemplateOut.from_dict(transformation_templates_item_data)

            transformation_templates.append(transformation_templates_item)

        settings = d.pop("settings")

        version = d.pop("version", UNSET)

        environment_out = cls(
            created_at=created_at,
            event_types=event_types,
            transformation_templates=transformation_templates,
            settings=settings,
            version=version,
        )

        environment_out.additional_properties = d
        return environment_out

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
