# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar

import attr

if TYPE_CHECKING:
    from ..models.event_type_schema_in_schema import EventTypeSchemaInSchema


T = TypeVar("T", bound="EventTypeSchemaIn")


@attr.s(auto_attribs=True)
class EventTypeSchemaIn:
    """
    Attributes:
        schema (EventTypeSchemaInSchema):  Example: {'description': 'An invoice was paid by a user', 'properties':
            {'invoiceId': {'description': 'The invoice id', 'type': 'string'}, 'userId': {'description': 'The user id',
            'type': 'string'}}, 'required': ['invoiceId', 'userId'], 'title': 'Invoice Paid Event', 'type': 'object'}.
    """

    schema: "EventTypeSchemaInSchema"
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        schema = self.schema

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "schema": schema,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        schema = d.pop("schema")

        event_type_schema_in = cls(
            schema=schema,
        )

        event_type_schema_in.additional_properties = d
        return event_type_schema_in

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
