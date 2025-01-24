# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar

import attr

from ..models.sink_out_type_1_type import SinkOutType1Type

T = TypeVar("T", bound="SinkOutType1")


@attr.s(auto_attribs=True)
class SinkOutType1:
    """
    Attributes:
        access_key (str):
        queue_dsn (str):
        region (str):
        secret_key (str):
        type (SinkOutType1Type):
    """

    access_key: str
    queue_dsn: str
    region: str
    secret_key: str
    type: SinkOutType1Type
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        access_key = self.access_key
        queue_dsn = self.queue_dsn
        region = self.region
        secret_key = self.secret_key
        type = self.type.value

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "accessKey": access_key,
                "queueDsn": queue_dsn,
                "region": region,
                "secretKey": secret_key,
                "type": type,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        access_key = d.pop("accessKey")

        queue_dsn = d.pop("queueDsn")

        region = d.pop("region")

        secret_key = d.pop("secretKey")

        type = SinkOutType1Type(d.pop("type"))

        sink_out_type_1 = cls(
            access_key=access_key,
            queue_dsn=queue_dsn,
            region=region,
            secret_key=secret_key,
            type=type,
        )

        sink_out_type_1.additional_properties = d
        return sink_out_type_1

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
