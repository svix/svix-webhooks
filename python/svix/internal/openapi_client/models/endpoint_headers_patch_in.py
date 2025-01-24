# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar

import attr

if TYPE_CHECKING:
    from ..models.endpoint_headers_patch_in_headers import EndpointHeadersPatchInHeaders


T = TypeVar("T", bound="EndpointHeadersPatchIn")


@attr.s(auto_attribs=True)
class EndpointHeadersPatchIn:
    """
    Attributes:
        headers (EndpointHeadersPatchInHeaders):  Example: {'X-Example': '123', 'X-Foobar': 'Bar'}.
    """

    headers: "EndpointHeadersPatchInHeaders"
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        headers = self.headers

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "headers": headers,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        headers = d.pop("headers")

        endpoint_headers_patch_in = cls(
            headers=headers,
        )

        endpoint_headers_patch_in.additional_properties = d
        return endpoint_headers_patch_in

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
