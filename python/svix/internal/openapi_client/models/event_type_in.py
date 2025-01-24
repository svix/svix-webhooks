# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.event_type_in_schemas import EventTypeInSchemas


T = TypeVar("T", bound="EventTypeIn")


@attr.s(auto_attribs=True)
class EventTypeIn:
    """
    Attributes:
        description (str):  Example: A user has signed up.
        name (str): The event type's name Example: user.signup.
        archived (Union[Unset, bool]):
        deprecated (Union[Unset, bool]):
        feature_flag (Union[Unset, None, str]):  Example: cool-new-feature.
        group_name (Union[Unset, None, str]): The event type group's name Example: user.
        schemas (Union[Unset, None, EventTypeInSchemas]): The schema for the event type for a specific version as a JSON
            schema. Example: {'1': {'description': 'An invoice was paid by a user', 'properties': {'invoiceId':
            {'description': 'The invoice id', 'type': 'string'}, 'userId': {'description': 'The user id', 'type':
            'string'}}, 'required': ['invoiceId', 'userId'], 'title': 'Invoice Paid Event', 'type': 'object'}}.
    """

    description: str
    name: str
    archived: Union[Unset, bool] = False
    deprecated: Union[Unset, bool] = False
    feature_flag: Union[Unset, None, str] = UNSET
    group_name: Union[Unset, None, str] = UNSET
    schemas: Union[Unset, None, "EventTypeInSchemas"] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        description = self.description
        name = self.name
        archived = self.archived
        deprecated = self.deprecated
        feature_flag = self.feature_flag
        group_name = self.group_name
        schemas = self.schemas

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "description": description,
                "name": name,
            }
        )
        if archived is not UNSET:
            field_dict["archived"] = archived
        if deprecated is not UNSET:
            field_dict["deprecated"] = deprecated
        if feature_flag is not UNSET:
            field_dict["featureFlag"] = feature_flag
        if group_name is not UNSET:
            field_dict["groupName"] = group_name
        if schemas is not UNSET:
            field_dict["schemas"] = schemas

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        description = d.pop("description")

        name = d.pop("name")

        archived = d.pop("archived", UNSET)

        deprecated = d.pop("deprecated", UNSET)

        feature_flag = d.pop("featureFlag", UNSET)

        group_name = d.pop("groupName", UNSET)

        schemas = d.pop("schemas", UNSET)

        event_type_in = cls(
            description=description,
            name=name,
            archived=archived,
            deprecated=deprecated,
            feature_flag=feature_flag,
            group_name=group_name,
            schemas=schemas,
        )

        event_type_in.additional_properties = d
        return event_type_in

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
