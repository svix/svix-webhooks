# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Optional, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.environment_in_settings import EnvironmentInSettings
    from ..models.event_type_in import EventTypeIn
    from ..models.template_in import TemplateIn


T = TypeVar("T", bound="EnvironmentIn")


@attr.s(auto_attribs=True)
class EnvironmentIn:
    """
    Attributes:
        event_types (Union[Unset, None, List['EventTypeIn']]):
        settings (Optional[EnvironmentInSettings]):
        transformation_templates (Union[Unset, None, List['TemplateIn']]):
    """

    settings: Optional["EnvironmentInSettings"]
    event_types: Union[Unset, None, List["EventTypeIn"]] = UNSET
    transformation_templates: Union[Unset, None, List["TemplateIn"]] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        event_types: Union[Unset, None, List[Dict[str, Any]]] = UNSET
        if not isinstance(self.event_types, Unset):
            if self.event_types is None:
                event_types = None
            else:
                event_types = []
                for event_types_item_data in self.event_types:
                    event_types_item = event_types_item_data.to_dict()

                    event_types.append(event_types_item)

        settings = self.settings
        transformation_templates: Union[Unset, None, List[Dict[str, Any]]] = UNSET
        if not isinstance(self.transformation_templates, Unset):
            if self.transformation_templates is None:
                transformation_templates = None
            else:
                transformation_templates = []
                for transformation_templates_item_data in self.transformation_templates:
                    transformation_templates_item = transformation_templates_item_data.to_dict()

                    transformation_templates.append(transformation_templates_item)

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "settings": settings,
            }
        )
        if event_types is not UNSET:
            field_dict["eventTypes"] = event_types
        if transformation_templates is not UNSET:
            field_dict["transformationTemplates"] = transformation_templates

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.event_type_in import EventTypeIn
        from ..models.template_in import TemplateIn

        d = src_dict.copy()
        event_types = []
        _event_types = d.pop("eventTypes", UNSET)
        for event_types_item_data in _event_types or []:
            event_types_item = EventTypeIn.from_dict(event_types_item_data)

            event_types.append(event_types_item)

        settings = d.pop("settings")

        transformation_templates = []
        _transformation_templates = d.pop("transformationTemplates", UNSET)
        for transformation_templates_item_data in _transformation_templates or []:
            transformation_templates_item = TemplateIn.from_dict(transformation_templates_item_data)

            transformation_templates.append(transformation_templates_item)

        environment_in = cls(
            event_types=event_types,
            settings=settings,
            transformation_templates=transformation_templates,
        )

        environment_in.additional_properties = d
        return environment_in

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
