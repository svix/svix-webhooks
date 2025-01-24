# TODO - remove this special case when we fix the generated code for empty openapi structs
import datetime
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union

import attr
from dateutil.parser import isoparse

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.event_type_out_schemas import EventTypeOutSchemas


T = TypeVar("T", bound="EventTypeOut")


@attr.s(auto_attribs=True)
class EventTypeOut:
    """
    Attributes:
        created_at (datetime.datetime):
        deprecated (bool):
        description (str):  Example: A user has signed up.
        name (str): The event type's name Example: user.signup.
        updated_at (datetime.datetime):
        archived (Union[Unset, bool]):
        feature_flag (Union[Unset, None, str]):  Example: cool-new-feature.
        group_name (Union[Unset, None, str]): The event type group's name Example: user.
        schemas (Union[Unset, None, EventTypeOutSchemas]): The schema for the event type for a specific version as a
            JSON schema. Example: {'1': {'description': 'An invoice was paid by a user', 'properties': {'invoiceId':
            {'description': 'The invoice id', 'type': 'string'}, 'userId': {'description': 'The user id', 'type':
            'string'}}, 'required': ['invoiceId', 'userId'], 'title': 'Invoice Paid Event', 'type': 'object'}}.
    """

    created_at: datetime.datetime
    deprecated: bool
    description: str
    name: str
    updated_at: datetime.datetime
    archived: Union[Unset, bool] = False
    feature_flag: Union[Unset, None, str] = UNSET
    group_name: Union[Unset, None, str] = UNSET
    schemas: Union[Unset, None, "EventTypeOutSchemas"] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        created_at = self.created_at.isoformat()

        deprecated = self.deprecated
        description = self.description
        name = self.name
        updated_at = self.updated_at.isoformat()

        archived = self.archived
        feature_flag = self.feature_flag
        group_name = self.group_name
        schemas = self.schemas

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "createdAt": created_at,
                "deprecated": deprecated,
                "description": description,
                "name": name,
                "updatedAt": updated_at,
            }
        )
        if archived is not UNSET:
            field_dict["archived"] = archived
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
        created_at = isoparse(d.pop("createdAt"))

        deprecated = d.pop("deprecated")

        description = d.pop("description")

        name = d.pop("name")

        updated_at = isoparse(d.pop("updatedAt"))

        archived = d.pop("archived", UNSET)

        feature_flag = d.pop("featureFlag", UNSET)

        group_name = d.pop("groupName", UNSET)

        schemas = d.pop("schemas", UNSET)

        event_type_out = cls(
            created_at=created_at,
            deprecated=deprecated,
            description=description,
            name=name,
            updated_at=updated_at,
            archived=archived,
            feature_flag=feature_flag,
            group_name=group_name,
            schemas=schemas,
        )

        event_type_out.additional_properties = d
        return event_type_out

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
