# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..models.kafka_security_protocol_type import KafkaSecurityProtocolType
from ..models.sink_out_type_2_type import SinkOutType2Type
from ..types import UNSET, Unset

T = TypeVar("T", bound="SinkOutType2")


@attr.s(auto_attribs=True)
class SinkOutType2:
    """
    Attributes:
        brokers (str):
        security_protocol (KafkaSecurityProtocolType):
        topic (str):
        type (SinkOutType2Type):
        sasl_password (Union[Unset, None, str]): Password for SASL, if `security_protocol` is `sasl-ssl`.
        sasl_username (Union[Unset, None, str]): Username for SASL, if `security_protocol` is `sasl-ssl`.
    """

    brokers: str
    security_protocol: KafkaSecurityProtocolType
    topic: str
    type: SinkOutType2Type
    sasl_password: Union[Unset, None, str] = UNSET
    sasl_username: Union[Unset, None, str] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        brokers = self.brokers
        security_protocol = self.security_protocol.value

        topic = self.topic
        type = self.type.value

        sasl_password = self.sasl_password
        sasl_username = self.sasl_username

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "brokers": brokers,
                "securityProtocol": security_protocol,
                "topic": topic,
                "type": type,
            }
        )
        if sasl_password is not UNSET:
            field_dict["saslPassword"] = sasl_password
        if sasl_username is not UNSET:
            field_dict["saslUsername"] = sasl_username

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        brokers = d.pop("brokers")

        security_protocol = KafkaSecurityProtocolType(d.pop("securityProtocol"))

        topic = d.pop("topic")

        type = SinkOutType2Type(d.pop("type"))

        sasl_password = d.pop("saslPassword", UNSET)

        sasl_username = d.pop("saslUsername", UNSET)

        sink_out_type_2 = cls(
            brokers=brokers,
            security_protocol=security_protocol,
            topic=topic,
            type=type,
            sasl_password=sasl_password,
            sasl_username=sasl_username,
        )

        sink_out_type_2.additional_properties = d
        return sink_out_type_2

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
