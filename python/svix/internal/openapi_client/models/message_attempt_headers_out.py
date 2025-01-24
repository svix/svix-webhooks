# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union, cast

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.message_attempt_headers_out_sent_headers import MessageAttemptHeadersOutSentHeaders


T = TypeVar("T", bound="MessageAttemptHeadersOut")


@attr.s(auto_attribs=True)
class MessageAttemptHeadersOut:
    """
    Attributes:
        sensitive (List[str]):
        sent_headers (MessageAttemptHeadersOutSentHeaders):
        response_headers (Union[Unset, None, List[List[str]]]):
    """

    sensitive: List[str]
    sent_headers: "MessageAttemptHeadersOutSentHeaders"
    response_headers: Union[Unset, None, List[List[str]]] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        sensitive = self.sensitive

        sent_headers = self.sent_headers
        response_headers: Union[Unset, None, List[List[str]]] = UNSET
        if not isinstance(self.response_headers, Unset):
            if self.response_headers is None:
                response_headers = None
            else:
                response_headers = []
                for response_headers_item_data in self.response_headers:
                    response_headers_item = response_headers_item_data

                    response_headers.append(response_headers_item)

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "sensitive": sensitive,
                "sentHeaders": sent_headers,
            }
        )
        if response_headers is not UNSET:
            field_dict["responseHeaders"] = response_headers

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        sensitive = cast(List[str], d.pop("sensitive"))

        sent_headers = d.pop("sentHeaders")

        response_headers = []
        _response_headers = d.pop("responseHeaders", UNSET)
        for response_headers_item_data in _response_headers or []:
            response_headers_item = cast(List[str], response_headers_item_data)

            response_headers.append(response_headers_item)

        message_attempt_headers_out = cls(
            sensitive=sensitive,
            sent_headers=sent_headers,
            response_headers=response_headers,
        )

        message_attempt_headers_out.additional_properties = d
        return message_attempt_headers_out

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
