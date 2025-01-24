# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, cast

import attr

if TYPE_CHECKING:
    from ..models.endpoint_headers_out_headers import EndpointHeadersOutHeaders


T = TypeVar("T", bound="EndpointHeadersOut")


@attr.s(auto_attribs=True)
class EndpointHeadersOut:
    """The value of the headers is returned in the `headers` field.

    Sensitive headers that have been redacted are returned in the sensitive field.

        Attributes:
            headers (EndpointHeadersOutHeaders):  Example: {'X-Example': '123', 'X-Foobar': 'Bar'}.
            sensitive (List[str]):  Example: ['Authorization'].
    """

    headers: "EndpointHeadersOutHeaders"
    sensitive: List[str]
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        headers = self.headers
        sensitive = self.sensitive

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "headers": headers,
                "sensitive": sensitive,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        headers = d.pop("headers")

        sensitive = cast(List[str], d.pop("sensitive"))

        endpoint_headers_out = cls(
            headers=headers,
            sensitive=sensitive,
        )

        endpoint_headers_out.additional_properties = d
        return endpoint_headers_out

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
