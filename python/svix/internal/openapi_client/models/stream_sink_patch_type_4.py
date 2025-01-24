# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar

import attr

from ..models.stream_sink_patch_type_4_type import StreamSinkPatchType4Type

if TYPE_CHECKING:
    from ..models.snowflake_config import SnowflakeConfig


T = TypeVar("T", bound="StreamSinkPatchType4")


@attr.s(auto_attribs=True)
class StreamSinkPatchType4:
    """
    Attributes:
        config (SnowflakeConfig): Configuration parameters for defining a Snowflake sink.
        type (StreamSinkPatchType4Type):
    """

    config: "SnowflakeConfig"
    type: StreamSinkPatchType4Type
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        config = self.config.to_dict()

        type = self.type.value

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "config": config,
                "type": type,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.snowflake_config import SnowflakeConfig

        d = src_dict.copy()
        config = SnowflakeConfig.from_dict(d.pop("config"))

        type = StreamSinkPatchType4Type(d.pop("type"))

        stream_sink_patch_type_4 = cls(
            config=config,
            type=type,
        )

        stream_sink_patch_type_4.additional_properties = d
        return stream_sink_patch_type_4

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
