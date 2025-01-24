# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union, cast

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.endpoint_transformation_simulate_in_payload import EndpointTransformationSimulateInPayload


T = TypeVar("T", bound="EndpointTransformationSimulateIn")


@attr.s(auto_attribs=True)
class EndpointTransformationSimulateIn:
    """
    Attributes:
        code (str):
        event_type (str): The event type's name Example: user.signup.
        payload (EndpointTransformationSimulateInPayload):
        channels (Union[Unset, None, List[str]]):
    """

    code: str
    event_type: str
    payload: "EndpointTransformationSimulateInPayload"
    channels: Union[Unset, None, List[str]] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        code = self.code
        event_type = self.event_type
        payload = self.payload
        channels: Union[Unset, None, List[str]] = UNSET
        if not isinstance(self.channels, Unset):
            if self.channels is None:
                channels = None
            else:
                channels = self.channels

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "code": code,
                "eventType": event_type,
                "payload": payload,
            }
        )
        if channels is not UNSET:
            field_dict["channels"] = channels

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        code = d.pop("code")

        event_type = d.pop("eventType")

        payload = d.pop("payload")

        channels = cast(List[str], d.pop("channels", UNSET))

        endpoint_transformation_simulate_in = cls(
            code=code,
            event_type=event_type,
            payload=payload,
            channels=channels,
        )

        endpoint_transformation_simulate_in.additional_properties = d
        return endpoint_transformation_simulate_in

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
