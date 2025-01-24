# TODO - remove this special case when we fix the generated code for empty openapi structs
import datetime
from typing import Any, Dict, List, Type, TypeVar, Union, cast

import attr
from dateutil.parser import isoparse

from ..models.transformation_template_kind import TransformationTemplateKind
from ..types import UNSET, Unset

T = TypeVar("T", bound="TemplateOut")


@attr.s(auto_attribs=True)
class TemplateOut:
    """
    Attributes:
        created_at (datetime.datetime):
        description (str):
        id (str):
        instructions (str):
        kind (TransformationTemplateKind):
        logo (str):
        name (str):
        org_id (str):
        transformation (str):
        updated_at (datetime.datetime):
        feature_flag (Union[Unset, None, str]):  Example: cool-new-feature.
        filter_types (Union[Unset, None, List[str]]):  Example: ['user.signup', 'user.deleted'].
        instructions_link (Union[Unset, None, str]):
    """

    created_at: datetime.datetime
    description: str
    id: str
    instructions: str
    kind: TransformationTemplateKind
    logo: str
    name: str
    org_id: str
    transformation: str
    updated_at: datetime.datetime
    feature_flag: Union[Unset, None, str] = UNSET
    filter_types: Union[Unset, None, List[str]] = UNSET
    instructions_link: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        created_at = self.created_at.isoformat()

        description = self.description
        id = self.id
        instructions = self.instructions
        kind = self.kind.value

        logo = self.logo
        name = self.name
        org_id = self.org_id
        transformation = self.transformation
        updated_at = self.updated_at.isoformat()

        feature_flag = self.feature_flag
        filter_types: Union[Unset, None, List[str]] = UNSET
        if not isinstance(self.filter_types, Unset):
            if self.filter_types is None:
                filter_types = None
            else:
                filter_types = self.filter_types

        instructions_link = self.instructions_link

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "createdAt": created_at,
                "description": description,
                "id": id,
                "instructions": instructions,
                "kind": kind,
                "logo": logo,
                "name": name,
                "orgId": org_id,
                "transformation": transformation,
                "updatedAt": updated_at,
            }
        )
        if feature_flag is not UNSET:
            field_dict["featureFlag"] = feature_flag
        if filter_types is not UNSET:
            field_dict["filterTypes"] = filter_types
        if instructions_link is not UNSET:
            field_dict["instructionsLink"] = instructions_link

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        created_at = isoparse(d.pop("createdAt"))

        description = d.pop("description")

        id = d.pop("id")

        instructions = d.pop("instructions")

        kind = TransformationTemplateKind(d.pop("kind"))

        logo = d.pop("logo")

        name = d.pop("name")

        org_id = d.pop("orgId")

        transformation = d.pop("transformation")

        updated_at = isoparse(d.pop("updatedAt"))

        feature_flag = d.pop("featureFlag", UNSET)

        filter_types = cast(List[str], d.pop("filterTypes", UNSET))

        instructions_link = d.pop("instructionsLink", UNSET)

        template_out = cls(
            created_at=created_at,
            description=description,
            id=id,
            instructions=instructions,
            kind=kind,
            logo=logo,
            name=name,
            org_id=org_id,
            transformation=transformation,
            updated_at=updated_at,
            feature_flag=feature_flag,
            filter_types=filter_types,
            instructions_link=instructions_link,
        )

        template_out.additional_properties = d
        return template_out

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
