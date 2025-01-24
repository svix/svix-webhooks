# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar

import attr

T = TypeVar("T", bound="MessageRawPayloadOut")


@attr.s(auto_attribs=True)
class MessageRawPayloadOut:
    """
    Attributes:
        payload (str):  Example: {"email":"test@example.com","type":"user.created","username":"test_user"}.
    """

    payload: str
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        payload = self.payload

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "payload": payload,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        payload = d.pop("payload")

        message_raw_payload_out = cls(
            payload=payload,
        )

        message_raw_payload_out.additional_properties = d
        return message_raw_payload_out

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
