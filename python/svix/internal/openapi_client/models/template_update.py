# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, Union, cast

import attr

from ..models.transformation_template_kind import TransformationTemplateKind
from ..types import UNSET, Unset

T = TypeVar("T", bound="TemplateUpdate")


@attr.s(auto_attribs=True)
class TemplateUpdate:
    """
    Attributes:
        logo (str):
        transformation (str):
        description (Union[Unset, str]):  Default: ''.
        feature_flag (Union[Unset, None, str]):  Example: cool-new-feature.
        filter_types (Union[Unset, None, List[str]]):  Example: ['user.signup', 'user.deleted'].
        instructions (Union[Unset, str]):  Default: ''.
        instructions_link (Union[Unset, None, str]):
        kind (Union[Unset, TransformationTemplateKind]):
        name (Union[Unset, str]):  Default: ''.
    """

    logo: str
    transformation: str
    description: Union[Unset, str] = ""
    feature_flag: Union[Unset, None, str] = UNSET
    filter_types: Union[Unset, None, List[str]] = UNSET
    instructions: Union[Unset, str] = ""
    instructions_link: Union[Unset, None, str] = UNSET
    kind: Union[Unset, TransformationTemplateKind] = UNSET
    name: Union[Unset, str] = ""
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        logo = self.logo
        transformation = self.transformation
        description = self.description
        feature_flag = self.feature_flag
        filter_types: Union[Unset, None, List[str]] = UNSET
        if not isinstance(self.filter_types, Unset):
            if self.filter_types is None:
                filter_types = None
            else:
                filter_types = self.filter_types

        instructions = self.instructions
        instructions_link = self.instructions_link
        kind: Union[Unset, str] = UNSET
        if not isinstance(self.kind, Unset):
            kind = self.kind.value

        name = self.name

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "logo": logo,
                "transformation": transformation,
            }
        )
        if description is not UNSET:
            field_dict["description"] = description
        if feature_flag is not UNSET:
            field_dict["featureFlag"] = feature_flag
        if filter_types is not UNSET:
            field_dict["filterTypes"] = filter_types
        if instructions is not UNSET:
            field_dict["instructions"] = instructions
        if instructions_link is not UNSET:
            field_dict["instructionsLink"] = instructions_link
        if kind is not UNSET:
            field_dict["kind"] = kind
        if name is not UNSET:
            field_dict["name"] = name

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        logo = d.pop("logo")

        transformation = d.pop("transformation")

        description = d.pop("description", UNSET)

        feature_flag = d.pop("featureFlag", UNSET)

        filter_types = cast(List[str], d.pop("filterTypes", UNSET))

        instructions = d.pop("instructions", UNSET)

        instructions_link = d.pop("instructionsLink", UNSET)

        _kind = d.pop("kind", UNSET)
        kind: Union[Unset, TransformationTemplateKind]
        if isinstance(_kind, Unset):
            kind = UNSET
        else:
            kind = TransformationTemplateKind(_kind)

        name = d.pop("name", UNSET)

        template_update = cls(
            logo=logo,
            transformation=transformation,
            description=description,
            feature_flag=feature_flag,
            filter_types=filter_types,
            instructions=instructions,
            instructions_link=instructions_link,
            kind=kind,
            name=name,
        )

        template_update.additional_properties = d
        return template_update

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
