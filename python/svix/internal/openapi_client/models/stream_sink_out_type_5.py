# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar

import attr

from ..models.stream_sink_out_type_5_type import StreamSinkOutType5Type

if TYPE_CHECKING:
    from ..models.google_cloud_storage_config import GoogleCloudStorageConfig


T = TypeVar("T", bound="StreamSinkOutType5")


@attr.s(auto_attribs=True)
class StreamSinkOutType5:
    """
    Attributes:
        config (GoogleCloudStorageConfig): Configuration for a Google Cloud Storage sink.

            Write stream events into the named bucket using the supplied Google Cloud credentials.
        type (StreamSinkOutType5Type):
    """

    config: "GoogleCloudStorageConfig"
    type: StreamSinkOutType5Type
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
        from ..models.google_cloud_storage_config import GoogleCloudStorageConfig

        d = src_dict.copy()
        config = GoogleCloudStorageConfig.from_dict(d.pop("config"))

        type = StreamSinkOutType5Type(d.pop("type"))

        stream_sink_out_type_5 = cls(
            config=config,
            type=type,
        )

        stream_sink_out_type_5.additional_properties = d
        return stream_sink_out_type_5

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
