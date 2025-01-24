# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

T = TypeVar("T", bound="IncomingWebhookPayloadOut")


@attr.s(auto_attribs=True)
class IncomingWebhookPayloadOut:
    """
    Attributes:
        channel (Union[Unset, None, str]):
        error (Union[Unset, None, str]):
        incoming_webhook_url (Union[Unset, None, str]):
    """

    channel: Union[Unset, None, str] = UNSET
    error: Union[Unset, None, str] = UNSET
    incoming_webhook_url: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        channel = self.channel
        error = self.error
        incoming_webhook_url = self.incoming_webhook_url

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if channel is not UNSET:
            field_dict["channel"] = channel
        if error is not UNSET:
            field_dict["error"] = error
        if incoming_webhook_url is not UNSET:
            field_dict["incomingWebhookUrl"] = incoming_webhook_url

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        channel = d.pop("channel", UNSET)

        error = d.pop("error", UNSET)

        incoming_webhook_url = d.pop("incomingWebhookUrl", UNSET)

        incoming_webhook_payload_out = cls(
            channel=channel,
            error=error,
            incoming_webhook_url=incoming_webhook_url,
        )

        incoming_webhook_payload_out.additional_properties = d
        return incoming_webhook_payload_out

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
