# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.sink_http_config_headers import SinkHttpConfigHeaders


T = TypeVar("T", bound="SinkHttpConfig")


@attr.s(auto_attribs=True)
class SinkHttpConfig:
    """
    Attributes:
        url (str):
        headers (Union[Unset, SinkHttpConfigHeaders]):
        key (Union[Unset, None, str]):
    """

    url: str
    headers: Union[Unset, "SinkHttpConfigHeaders"] = UNSET
    key: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        url = self.url
        headers = self.headers
        key = self.key

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "url": url,
            }
        )
        if headers is not UNSET:
            field_dict["headers"] = headers
        if key is not UNSET:
            field_dict["key"] = key

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        url = d.pop("url")

        headers = d.pop("headers", UNSET)

        key = d.pop("key", UNSET)

        sink_http_config = cls(
            url=url,
            headers=headers,
            key=key,
        )

        sink_http_config.additional_properties = d
        return sink_http_config

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
